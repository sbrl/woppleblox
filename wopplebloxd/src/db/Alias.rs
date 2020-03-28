use time::Timespec;

#[derive(Debug)]
pub struct Alias {
    id : i64,
    owner_id : i64,
    date_created : Timespec,
    name : String,
    profile_description : String,
    avatar_filename : String
}
