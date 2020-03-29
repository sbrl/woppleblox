use chrono::{ DateTime, Utc };
use chrono::serde::ts_milliseconds;
use serde_derive::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id : i64,
    pub alias_id : i64,
    #[serde(with = "ts_milliseconds")]
    pub date_created : DateTime<Utc>,
    pub content : String,
}
