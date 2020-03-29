use rusqlite::{ CachedStatement, Result, ToSql, MappedRows };
use pinto::query_builder;
use serde_rusqlite::from_rows;

use crate::db::definitions::{ Connection };
use crate::db::{ User };

// #[macro_export]
// macro_rules! prepare_sql {
//     ( $table:expr, $query:expr ) => {
//         {
//             conn.prepare(&format!($query, $table).to_string())
//         }
//     }
// }

pub struct UserRepository;

impl UserRepository {
    const TABLE_NAME : &'static str = "users";
    
    fn new(conn : Connection) -> UserRepository {
        UserRepository {
            
        }
    }
    
    pub fn query_get_by_id(conn : Connection, id : i64) -> Result<User> {
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("id == :id")
                .build().to_string()
        )?;
        let mut result = from_rows::<User>(stmt.query_named(named_params!{
            ":id": id
        })?);
        Ok(result.next().unwrap().unwrap())
    }
    
    pub fn query_get_by_username(conn : Connection, username : String) -> Result<User> {
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("username == :username")
                .build().to_string()
        )?;
        let mut result = from_rows::<User>(stmt.query_named(named_params!{
            ":username": username
        })?);
        Ok(result.next().unwrap().unwrap())
    }
}
