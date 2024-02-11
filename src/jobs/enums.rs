use postgres_types::{FromSql, ToSql};
use serde::Serialize;

#[derive(Debug, ToSql, FromSql, Serialize)]
#[postgres(name = "jop_type")]
pub enum JopType {
    #[postgres(name = "printing")]
    Printing,
    #[postgres(name = "copying")]
    Copying,
}

#[derive(Debug, ToSql, FromSql, Serialize)]
#[postgres(name = "sides")]
pub enum Sides {
    #[postgres(name = "one-side")]
    OneSide,
    #[postgres(name = "two-sides")]
    TwoSides,
}

#[derive(Debug, ToSql, FromSql, Serialize)]
#[postgres(name = "paper_wight")]
pub enum PaperWight {
    #[postgres(name = "70g")]
    G70,
    #[postgres(name = "80g")]
    G80,
}

#[derive(Debug, ToSql, FromSql, Serialize)]
#[postgres(name = "printing_quality")]
pub enum PrintingQuality {
    #[postgres(name = "standard")]
    HighQuality,
    #[postgres(name = "high-quality")]
    Standard,
}
