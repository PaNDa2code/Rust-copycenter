use crate::config::the_client;
use crate::{files::*, Customer, User};
use std::fmt::Debug;
use std::mem;
use time::PrimitiveDateTime;

mod enums;
use enums::*;




#[derive(Debug)]
pub struct Job {
    pub jop_id: Option<i32>,
    pub teacher: Customer,
    pub user: User,
    pub jop_added_at_time: Option<PrimitiveDateTime>,
    pub jop_done_at_time: Option<PrimitiveDateTime>,
    pub jop_type: JopType,
    pub jop_done: bool,
    pub file: Option<PrintingFile>,
    pub pages_per_sheet: i32,
    pub paper_wight: PaperWight,
    pub copies_count: i32,
    pub paper_count: i32,
    pub sides: Sides,
    pub plank_back_cover: bool,
    pub printing_quality: PrintingQuality,
}


impl Job {
    pub fn from_row(row: &postgres::Row) -> Self {
        let file_id: Option<i32> = row.get(11);
        let file: Option<PrintingFile> =
            match file_id {
                Some(id) => {
                    Some(PrintingFile {
                        file_id: id,
                        file_name: row.get(12),
                        file_checksum_sha_256: row.get(13),
                        file_type: row.get(14),
                        file_dir: row.get(15),
                        file_pages_count: row.get(16),
                    })
                },
                None => None, // Handle the case when file_id is None (NULL in the database)
            };
        Job {
            jop_id: row.get(0),
            teacher: Customer {
                customer_id: row.get(1),
                customer_name: row.get(2),
                customer_phone_number: row.get(3)
            },
            user: User {
                id: row.get(4),
                full_name: row.get(5),
                username: row.get(6)
            },
            jop_added_at_time: row.get(7),
            jop_done_at_time: row.get(8),
            jop_type: row.get(9),
            jop_done: row.get(10),
            file: file,
            paper_wight: row.get(17),
            paper_count: row.get(18),
            sides: row.get(19),
            plank_back_cover: row.get(20),
            printing_quality: row.get(21),
            pages_per_sheet: row.get(22),
            copies_count: row.get(23)
        }
    }

    pub fn total_size(&self) -> usize {
        mem::size_of::<Self>()
    }

    pub fn insert_job(&self) -> Result<(), postgres::Error> {
        let query = "
            INSERT INTO jobs(
                teacher_id,
                user_id,
                jop_type,
                file_id,
                pages_per_sheet,
                paper_wight,
                copies_count,
                paper_count,
                sides,
                plank_back_cover,
                printing_quality
            )
            VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11);
        ";

        let mut client = the_client()?;
        
        let file_id = match &self.file {
            Some(file) => {Some(file.file_id)}
            None => {None}
        };

        client.execute(query, &[
            &self.teacher.customer_id,
            &self.user.id,
            &self.jop_type,
            &file_id,
            &self.pages_per_sheet,
            &self.paper_wight,
            &self.copies_count,
            &self.paper_count,
            &self.sides,
            &self.plank_back_cover,
            &self.printing_quality
        ]).expect("Error inserting to jobs");

        Ok(())

    }
}

pub fn fetch_jobs() -> Result<Vec<Job>, postgres::Error> {
    let query = 
            "SELECT
                jop_id,
                teacher_id,
                teacher_name,
                teacher_phone_number,
                user_id,
                user_full_name,
                user_name,
                jop_added_at_time,
                jop_done_at_time,
                jop_type,
                jop_done,
                file_id,
                file_name,
                file_checksum_sha_256,
                file_type,
                file_dir,
                file_pages_count,
                paper_wight,
                paper_count,
                sides,
                plank_back_cover,
                printing_quality,
                pages_per_sheet,
                copies_count
            FROM
                jobs
            JOIN
                teachers USING(teacher_id)
            JOIN
                users USING(user_id)
            LEFT JOIN
                files USING(file_id);
            ";  
    let mut client = the_client()?;
    let rows = client.query(query, &[])?;
    let jobs: Vec<Job> = rows.iter().map(|row| Job::from_row(row)).collect();

    Ok(jobs)
}


