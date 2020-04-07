use std::result::Result;

pub trait StringResult<T> {
    fn make_normal(&self) -> std::result::Result<T, String>;
}

impl<T> StringResult<T> for Result<T, String> {
    fn make_normal(&self) -> std::result::Result<T, String> {
        let copy = self.clone();
        match copy {
            Ok(val) => std::result::Result::Ok(val: T),
            Err(error) => std::result::Result::Err(error.to_string())
        }
    }
}
