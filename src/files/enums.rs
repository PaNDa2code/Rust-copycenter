use postgres_types::{FromSql, ToSql};
use serde::Serialize;

#[derive(Debug, ToSql, FromSql, Serialize)]
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
