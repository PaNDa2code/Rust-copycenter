pub mod config;
pub mod errors_handle;
pub mod files;
pub mod jobs;
pub mod users;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tt() {
        let file = files::PrintingFile::new("/media/panda/New Volume/code/linux/rust/copycenter/src-tauri/my_lib/test_dir/printing_files/target_pdf/blank.pdf").unwrap();
        println!("{:?}", file);
    }
}
