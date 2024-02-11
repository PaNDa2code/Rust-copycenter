use crate::config::the_client;
use postgres::Error;
use serde::Serialize;

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub username: String,
}

impl User {
    fn from_row(qr: &postgres::Row) -> User {
        User {
            id: qr.get(0),
            full_name: qr.get(1),
            username: qr.get(2),
        }
    }

    pub fn by_username(username: &str) -> Result<User, postgres::Error> {
        let mut client = the_client()?;

        let query = "
            SELECT
                *
            FROM
                users
            WHERE
                user_name = $1
            ;
            ";
        let row = client.query_one(query, &[&username])?;
        Ok(User::from_row(&row))
    }
}

#[derive(Debug, Serialize)]
pub struct Customer {
    pub customer_id: Option<i32>,
    pub customer_name: String,
    pub customer_phone_number: String,
}

pub fn fetch_users() -> Result<Vec<User>, Error> {
    let mut client = the_client()?;
    let rows = client.query("SELECT * FROM users;", &[])?;
    let users: Vec<User> = rows.iter().map(|row| User::from_row(row)).collect();
    Ok(users)
}
