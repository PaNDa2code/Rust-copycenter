use postgres_types::{FromSql, ToSql};

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
    pub file_id: i32,
    pub file_type: FileType,
    pub file_checksum_sha_256: String,
    pub file_name: String,
    pub file_dir: String,
    pub file_pages_count: i32
}