use serde_derive::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    pub id : i64,
    pub post_id : i64,
    pub filename : String
}
