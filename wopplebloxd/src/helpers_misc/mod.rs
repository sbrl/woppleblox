pub trait NormalResult {
    fn make_normal<T>(&self) -> std::Result<T, String>;
}

impl NormalResult for rusqlite::Result {
    fn make_normal<T>(&self) -> std::Result<T, String> {
        match self {
            rusqlite::Result::Ok(val) => std::Result::Ok(val),
            rusqlite::Result::Err(error) =>  std::Result::Err(error.to_tring())
        }
    }
}
