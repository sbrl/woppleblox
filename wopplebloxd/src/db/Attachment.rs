use time::Timespec;

#[derive(Debug)]
pub struct Attachment {
    pub id : i64,
    pub post_id : i64,
    pub filename : Timespec
}
