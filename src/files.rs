use postgres_types::{ToSql, FromSql};
use crate::config::the_client;

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "file_type")]
pub enum FileType {
    #[postgres(name = "pdf")]
    PDF,
    #[postgres(name = "word")]
    Word,
    #[postgres(name = "image")]
    Image,
    #[postgres(name = "other")]
    Other,
}

#[derive(Debug, ToSql, FromSql,)]
pub struct PrintingFile {
    pub file_id: Option<i32>,
    pub file_type: FileType,
    pub file_checksum_sha_264: String,
    pub file_name: String,
    pub file_dir: String,
    pub file_pages_count: i32
}


impl PrintingFile {
    pub fn from_row(row: &postgres::Row) -> Self {
        PrintingFile {
            file_id: row.get(0),
            file_type: row.get(1),
            file_checksum_sha_264: row.get(2),
            file_name: row.get(3),
            file_dir: row.get(4),
            file_pages_count: row.get(5),
        }
    }
    pub fn insert_file(&self) -> Result<(), postgres::Error>{
        let mut client = the_client()?;
        let query = "
            INSERT INTO files(
                file_type, file_checksum_SHA_264, file_name, file_dir, file_pages_count
            )
            VALUES(
                $1, $2, $3, $4, $5
            )
            ";
    
        client.execute(query, &[
            &self.file_type,
            &self.file_checksum_sha_264,
            &self.file_name,
            &self.file_dir,
            &self.file_pages_count,
            ])?;
        
        Ok(())
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
    let files: Vec<PrintingFile> = rows.iter().map(|row| PrintingFile::from_row(row)).collect();

    Ok(files)
}