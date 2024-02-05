use crate::files::*;

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