use rusqlite::{ CachedStatement, NO_PARAMS, RowIndex };

pub trait StatementHelper {
    fn query_value(&mut self) -> Result<String, String>;
}

impl StatementHelper for CachedStatement<'_> {
    fn query_value(&mut self) -> Result<String, String> {
        let mut rows = match self.query(NO_PARAMS) {
            Ok(rows) => rows,
            Err(err) => return Err(err.to_string())
        };
        
        match rows.next() {
            Err(err) => return Err(err.to_string()),
            Ok(row) => match row {
                None => Err("Error: No value returned by SQLite (something weird is going on here.....)".to_string()),
                Some(val) => Ok(val.get(0).unwrap())
            }
        }
    }
}
