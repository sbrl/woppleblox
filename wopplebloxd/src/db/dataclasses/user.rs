use chrono::{ DateTime, Utc };

#[derive(Debug)]
pub struct User {
    id : i64,
    username : String,
    password : String,
    date_created : DateTime<Utc>
}
