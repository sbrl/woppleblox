use time::Timespec;

#[derive(Debug)]
pub struct Post {
    pub id : i64,
    pub alias_id : i64,
    pub date_created : Timespec,
    pub content : String,
}
