use crate::{config::the_client, files::*};
use pdf::file::FileOptions;

use sha256::{self, try_digest};
use std::{path::Path};

impl PrintingFile {
    fn from_row(row: &postgres::Row) -> Self {
        PrintingFile {
            file_id: row.get(0),
            file_type: row.get(1),
            file_checksum_sha_256: row.get(2),
            file_name: row.get(3),
            file_dir: row.get(4),
            file_pages_count: row.get(5),
        }
    }

    pub fn file_by_checksum(file_checksum_sha_256: &str) -> Option<PrintingFile>{
        let query = "
            SELECT
                file_id, file_type, file_name, file_dir, file_pages_count
            FROM
                files
            WHERE
                file_checksum_sha_256 = $1
            ;
        ";

        let mut client = the_client().unwrap();

        let row = client.query_opt(query, &[&file_checksum_sha_256]).unwrap();

        match row {
            Some(r) => {
                Some(PrintingFile {
                    file_id: r.get(0),
                    file_type: r.get(1),
                    file_checksum_sha_256: file_checksum_sha_256.to_string(),
                    file_name: r.get(2),
                    file_dir: r.get(3),
                    file_pages_count: r.get(4)
                })
            }
            None => None
        }
    }
    pub fn file_checksum_sha_256(file_path: &str) -> Result<String, std::io::Error> {
        let file_path = Path::new(file_path);
        let file_checksum = try_digest(file_path);

        match file_checksum {
            Ok(cs) => {Ok(cs)}
            Err(err) => { 
                eprintln!("Error generating file checksum: {}", err);
                Err(err)
            }
        }
    }

    pub fn count_pages(pdf_path: &str) -> Result<i32, pdf::error::PdfError> {
        let file = FileOptions::cached().open(pdf_path)?;
        let file_pages_count = file.num_pages().try_into().unwrap();
        Ok(file_pages_count)
    }
}

pub fn fetch_files() -> Result<Vec<PrintingFile>, postgres::Error> {
    let query = 
            "SELECT
                *
            FROM
                files;
            ";  
    let mut client = the_client()?;
    let rows = client.query(query, &[])?;
    let files = rows.iter().map(|row| PrintingFile::from_row(row)).collect();

    Ok(files)
}