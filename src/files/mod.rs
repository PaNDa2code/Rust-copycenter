mod enums;
mod functions;
mod new;
pub mod errors;



// use functions::*;
pub use enums::*;
pub use new::*;


#[derive(Debug)]
pub struct PrintingFile {
    pub file_id: i32,
    pub file_type: FileType,
    pub file_checksum_sha_256: String,
    pub file_name: String,
    pub file_dir: String,
    pub file_pages_count: i32
}

