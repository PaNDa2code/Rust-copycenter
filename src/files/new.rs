use crate::files::*;

use std::{fs::{copy, create_dir, remove_file}, path::Path};
use crate::{config::the_client, config::STORAGE_PATH};

impl PrintingFile {
    pub fn new(file_path: &str) -> Result<Self, u8> {
        let path = Path::new(file_path);

        let file_ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");

        let file_type = match file_ext {
            "pdf" => FileType::PDF,
            "docx" => FileType::Word,
            "jpg" | "jpeg" => FileType::Image,
            _ => FileType::Other
        };

        let file_checksum_sha_256 = Self::file_checksum_sha_256(file_path);
        let file_name = path.file_name().and_then(|x| x.to_str()).unwrap().to_string();
        let mut file_dir = path.parent().and_then(|x| x.to_str()).unwrap().to_string();
        let file_pages_count = Self::count_pages(file_path).unwrap();


        let file_check = Self::file_by_checksum(&file_checksum_sha_256);

        if file_check.is_some() {
            let file = file_check.unwrap();
            eprintln!("Error: File is already there ==> {:?}", file);
            return Err(50);
        }

        // move the file to the storage

        let store_dir = format!("{}/{}", STORAGE_PATH, file_checksum_sha_256);
        let store_dir_path = Path::new(&store_dir);
        println!("{:?}", store_dir_path);
        match create_dir(&store_dir_path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error creating storage dir: {}", err);
                return Err(22);
            }
        }

        let store_file_path = store_dir_path.join(&file_name);
        let current_file_path = Path::new(&file_dir).join(&file_name);

        match copy(&current_file_path, &store_file_path) {
            Ok(_) =>  {}
            Err(err) => {
                eprintln!("Error copying storage dir: {}", err);
                return Err(27);
            }
        }

        match remove_file(&current_file_path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Can't remove the file from it's old path: {}", err);
                return Err(41);
            }
        }

        file_dir = store_file_path.to_str().unwrap().to_string();


        let mut client = the_client().unwrap();

        let query = "
            INSERT INTO
                files(
                    file_name, file_dir, file_checksum_SHA_256, file_pages_count
                )
                VALUES(
                    $1, $2, $3, $4
                )
            RETURNING
                file_id
            ;
        ";
        let row = client.query_one(query, &[
            &file_name,
            &file_dir,
            &file_checksum_sha_256,
            &file_pages_count
        ]);

        println!("{:?}", row);

        match row {
            Ok(r) => {
                Ok(
                    PrintingFile{
                        file_id: r.get(0),
                        file_name,
                        file_dir,
                        file_checksum_sha_256,
                        file_type,
                        file_pages_count
                    }
                )
            }
            Err(err) => {
                eprintln!("Error retrieving the row: {}", err);
                Err(60)
            }
        }
        
    }
}