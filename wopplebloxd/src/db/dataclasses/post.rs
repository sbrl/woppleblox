use chrono::{ DateTime, Utc };

#[derive(Debug)]
pub struct Post {
    pub id : i64,
    pub alias_id : i64,
    pub date_created : DateTime<Utc>,
    pub content : String,
}
