use chrono::{ DateTime, Utc };

#[derive(Debug)]
pub struct Alias {
    pub id : i64,
    pub owner_id : i64,
    pub date_created : DateTime<Utc>,
    pub name : String,
    pub profile_description : String,
    pub avatar_filename : String
}
