mod users;
mod files;
mod jobs;
mod config;




#[allow(unused_imports)]
use users::*;
#[allow(unused_imports)]
use files::{PrintingFile, errors};
#[allow(unused_imports)]
use jobs::*;

fn main(){
    let file1 = PrintingFile::new("/media/panda/New Volume/code/linux/rust/postgers/test_dir/printing_files/target_pdf/3000.pdf");
    let file2 = PrintingFile::new("/media/panda/New Volume/code/linux/rust/postgers/test_dir/printing_files/target_pdf/blank.pdf");
    let file3 = PrintingFile::new("/media/panda/New Volume/code/linux/rust/postgers/test_dir/printing_files/target_pdf/document.pdf");
    println!("{:?}", file1);
    println!("{:?}", file2);
    println!("{:?}", file3);
}


