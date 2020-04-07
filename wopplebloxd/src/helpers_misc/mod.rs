pub trait NormalResult<T> {
    fn make_normal<T>(&self) -> std::result::Result<T, String>;
}

impl NormalResult<T> for rusqlite::Result<T> {
    fn make_normal<T>(&self) -> std::result::Result<T, String> {
        match self {
            rusqlite::Result::Ok(val) => std::result::Result::Ok(val),
            rusqlite::Result::Err(error) =>  std::Result::Err(error.to_tring())
        }
    }
}
