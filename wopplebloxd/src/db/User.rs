use time::Timespec;

#[derive(Debug)]
pub struct User {
    id : i64,
    username : String,
    password : String,
    date_created : Timespec
}
