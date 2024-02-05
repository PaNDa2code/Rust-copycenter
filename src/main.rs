#![allow(dead_code)]

use std::{fs::File, io::Write};


mod users;
mod files;
mod jobs;
mod config;


#[allow(unused_imports)]
use users::*;
#[allow(unused_imports)]
use files::*;
#[allow(unused_imports)]
use jobs::*;

fn main(){

    let file = PrintingFile::new("/home/panda/Downloads/document.pdf")
                .expect("Can't make new PrintingFile");
    println!("{:?}", file);
}


