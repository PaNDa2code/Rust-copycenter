mod enums;
mod functions;
mod new;
pub mod errors;



use functions::*;
pub use enums::*;
pub use new::*;
use std::io::Read;
use pdf::file::FileOptions;
use sha256;
use crate::config::the_client;


impl PrintingFile {

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
    
    fn file_checksum_sha_256(file_path: &str) -> String {
        let mut file = std::fs::File::open(file_path).expect("Failed to open file");
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer).expect("Failed to read file");
        sha256::digest_bytes(&file_buffer)
    }

    fn count_pages(pdf_path: &str) -> Result<i32, pdf::error::PdfError> {
        let file = FileOptions::cached().open(pdf_path)?;
        let file_pages_count = file.num_pages().try_into().unwrap();
        Ok(file_pages_count)
    }
}

