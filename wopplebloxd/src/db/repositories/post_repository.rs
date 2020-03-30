use rusqlite::{ CachedStatement, Result, ToSql, MappedRows };
use pinto::query_builder;
use serde_rusqlite::{ from_rows, to_params_named };

use crate::db::definitions::{ Connection };
use crate::db::{ Alias, Post };

// #[macro_export]
// macro_rules! prepare_sql {
//     ( $table:expr, $query:expr ) => {
//         {
//             conn.prepare(&format!($query, $table).to_string())
//         }
//     }
// }

pub struct PostRepository;

impl PostRepository {
    const TABLE_NAME : &'static str = "posts";
    
    fn new(conn : Connection) -> PostRepository {
        PostRepository {
            
        }
    }
    
    pub fn get_by_id(conn : Connection, id : i64) -> Result<Post> {
        // FUTURE: Write a macro to cut down on the boilerplate here
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("id == :id")
                .build().to_string()
        )?;
        let mut result = from_rows::<Post>(stmt.query_named(named_params!{
            ":id": id
        })?);
        Ok(result.next().unwrap().unwrap())
    }
    
    pub fn get_by_alias(conn : Connection, alias : Alias) -> Result<Vec<Post>> {
        Self::get_by_alias_id(conn, alias.id)
    }
    
    pub fn get_by_alias_id(conn : Connection, alias_id : i64) -> Result<Vec<Post>> {
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("alias_id == :alias_id")
                .build().to_string()
        )?;
        let query_result = from_rows::<Post>(stmt.query_named(named_params!{
            ":alias_id": alias_id
        })?);
        let mut result = Vec::<Post>::new();
        for alias in query_result {
            result.push(alias.unwrap());
        }
        Ok(result)
    }
    
    pub fn save(conn : Connection, alias : Post) -> Result<usize> {
        // This might not work as intended - it depends on how pinto builds the query
        conn.execute_named(
            &format!("INSERT OR REPLACE {}
                (id, alias_id, date_created, date_modified, content)
                VALUES (:id, :alias_id, :date_created, :date_modified, :content)", Self::TABLE_NAME),
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
    
    pub fn delete(conn : Connection, user : Post) -> Result<usize> {
        Self::delete_by_id(conn, user.id)
    }
}
