use chrono::{ DateTime, Utc };
use chrono::serde::ts_milliseconds;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id : i64,
    pub username : String,
    pub password : String,
    #[serde(with = "ts_milliseconds")]
    pub date_created : DateTime<Utc>
}
