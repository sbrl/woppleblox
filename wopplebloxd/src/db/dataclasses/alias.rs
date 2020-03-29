use chrono::{ DateTime, Utc };
use chrono::serde::ts_milliseconds;
use serde_derive::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    pub id : i64,
    pub owner_id : i64,
    #[serde(with = "ts_milliseconds")]
    pub date_created : DateTime<Utc>,
    pub name : String,
    pub profile_description : String,
    pub avatar_filename : String
}
