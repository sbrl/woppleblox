use rusqlite::{ CachedStatement, Result, ToSql, MappedRows };
use pinto::query_builder;
use serde_rusqlite::{ from_rows, to_params_named };

use crate::helpers_data::db_conn::StatementHelper;
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
    
    pub fn have_users(conn : Connection) -> Result<bool> {
        let mut stmt = conn.prepare_cached(
            &format!("SELECT exists (SELECT 1 from {})", Self::TABLE_NAME)
        )?;
        Ok(match stmt.query_value().unwrap().as_ref() {
            "1" => true,
            _ => false
        })
    }
    
    pub fn get_by_id(conn : Connection, id : i64) -> Result<User> {
        // FUTURE: Write a macro to cut down on the boilerplate here
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
    
    pub fn get_by_username(conn : Connection, username : String) -> Result<User> {
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
    
    pub fn save(conn : Connection, user : User) -> Result<usize> {
        // This might not work as intended - it depends on how pinto builds the query
        conn.execute_named(
            &format!("INSERT OR REPLACE {}
                (id, username, password, date_created)
                VALUES (:id, :username, :password, :date_created)", Self::TABLE_NAME),
            // &query_builder::insert(Self::TABLE_NAME)
            // .set("id", ":id")
            // .set("username", ":username")
            // .set("password", ":password")
            // .set("date_created", ":date_created")
            // .build().to_string(),
            &to_params_named(&user).unwrap().to_slice()
        )
    }
    
    pub fn delete_by_id(conn : Connection, id : i64) -> Result<usize> {
        conn.execute_named(
            &query_builder::delete(Self::TABLE_NAME)
                .filter("id = :id")
                .build(),
            named_params!{
                ":id": id
            }
        )
    }
    
    pub fn delete(conn : Connection, user : User) -> Result<usize> {
        Self::delete_by_id(conn, user.id)
    }
}
