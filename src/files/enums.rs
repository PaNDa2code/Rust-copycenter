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