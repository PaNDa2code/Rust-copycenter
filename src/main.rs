#![allow(dead_code)]

mod users;
mod config;
mod jobs;
mod files;

#[allow(unused_imports)]
use users::*;
#[allow(unused_imports)]
use files::*;
#[allow(unused_imports)]
use jobs::*;

fn main(){
    // println!("Start the app");
    // match fetch_jobs() {
    //     Ok(jobs) => {
    //         for job in jobs {
    //             println!("\n{:?}", job);
    //             println!("Size: {}", job.total_size());
    //         }
    //     }
    //     Err(err) => eprintln!("Error fetching jobs: {}", err),
    // }
    
    // let file = PrintingFile {
    //     file_type: files::FileType::PDF,
    //     file_name: String::from("test.pdf"),
    //     file_dir: String::from("/path/to/pdf"),
    //     file_checksum_sha_264: String::from("241hfd9h29e1ug8hd8"),
    //     file_pages_count: 182,
    //     file_id: None
    // };

    // match file.insert_file() {
        // Ok(_b) => {println!("inserted file successfully")}
        // Err(err) => eprintln!("Error inserting file: {}", err)
    // }
    // let teacher = Teacher {
    //     teacher_id:Some(1),
    //     teacher_name:String::from("John Doe"),
    //     teacher_phone_number:String::from("01234567890"),
    // };
    // let user = User {
    //     user_id: 1,
    //     user_full_name:String::from("Alice Wonderland"),
    //     user_name:String::from("alice"),
    // };

    // let job = Job {
    //     jop_id: None,
    //     teacher,
    //     user,
    //     jop_added_at_time:None,
    //     jop_done_at_time: None,
    //     jop_type:JopType::Copying,
    //     jop_done:false,
    //     file: Some(file),
    //     pages_per_sheet:1,
    //     paper_wight: PaperWight::G70,
    //     copies_count: 10,
    //     paper_count: 500,
    //     sides: Sides::TwoSides,
    //     plank_back_cover: true,
    //     printing_quality:PrintingQuality::Standard,
    // };
    // job.insert_job();
    let user = User::by_username("alice").expect("Can't find username");
    let user2 =  User::by_username("bob").expect("Can't find username");
    let user_json = serde_json::to_string(&[user,user2]).expect("Failed to serialize User to JSON");

    println!("{}", user_json);
    
}


