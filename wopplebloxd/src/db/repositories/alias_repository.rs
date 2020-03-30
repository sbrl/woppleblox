use rusqlite::{ CachedStatement, Result, ToSql, MappedRows };
use pinto::query_builder;
use serde_rusqlite::{ from_rows, to_params_named };

use crate::db::definitions::{ Connection };
use crate::db::{ User, Alias };

// #[macro_export]
// macro_rules! prepare_sql {
//     ( $table:expr, $query:expr ) => {
//         {
//             conn.prepare(&format!($query, $table).to_string())
//         }
//     }
// }

pub struct AliasRepository;

impl AliasRepository {
    const TABLE_NAME : &'static str = "aliases";
    
    fn new(conn : Connection) -> AliasRepository {
        AliasRepository {
            
        }
    }
    
    pub fn get_by_id(conn : Connection, id : i64) -> Result<Alias> {
        // FUTURE: Write a macro to cut down on the boilerplate here
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("id == :id")
                .build().to_string()
        )?;
        let mut result = from_rows::<Alias>(stmt.query_named(named_params!{
            ":id": id
        })?);
        Ok(result.next().unwrap().unwrap())
    }
    
    pub fn get_by_user(conn : Connection, user : User) -> Result<Vec<Alias>> {
        Self::get_by_user_id(conn, user.id)
    }
    
    pub fn get_by_user_id(conn : Connection, user_id : i64) -> Result<Vec<Alias>> {
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("owner_id == :owner_id")
                .build().to_string()
        )?;
        let query_result = from_rows::<Alias>(stmt.query_named(named_params!{
            ":owner_id": user_id
        })?);
        let mut result = Vec::<Alias>::new();
        for alias in query_result {
            result.push(alias.unwrap());
        }
        Ok(result)
    }
    
    pub fn save(conn : Connection, alias : Alias) -> Result<usize> {
        // This might not work as intended - it depends on how pinto builds the query
        conn.execute_named(
            &format!("INSERT OR REPLACE {}
                (id, name, owner_id, date_created, profile_description, avatar_filename)
                VALUES (:id, :name, :owner_id, :date_created, :profile_description, :avatar_filename)", Self::TABLE_NAME),
            // &query_builder::insert(Self::TABLE_NAME)
            // .set("id", ":id")
            // .set("username", ":username")
            // .set("password", ":password")
            // .set("date_created", ":date_created")
            // .build().to_string(),
            &to_params_named(&alias).unwrap().to_slice()
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
    
    pub fn delete(conn : Connection, user : Alias) -> Result<usize> {
        Self::delete_by_id(conn, user.id)
    }
}
