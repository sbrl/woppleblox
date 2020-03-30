use rusqlite::{ CachedStatement, Result, ToSql, MappedRows };
use pinto::query_builder;
use serde_rusqlite::{ from_rows, to_params_named };

use crate::db::definitions::{ Connection };
use crate::db::{ Post, Attachment };

// #[macro_export]
// macro_rules! prepare_sql {
//     ( $table:expr, $query:expr ) => {
//         {
//             conn.prepare(&format!($query, $table).to_string())
//         }
//     }
// }

pub struct AttachmentRepository;

impl AttachmentRepository {
    const TABLE_NAME : &'static str = "attachments";
    
    fn new(conn : Connection) -> AttachmentRepository {
        AttachmentRepository {
            
        }
    }
    
    pub fn get_by_id(conn : Connection, id : i64) -> Result<Attachment> {
        // FUTURE: Write a macro to cut down on the boilerplate here
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("id == :id")
                .build().to_string()
        )?;
        let mut result = from_rows::<Attachment>(stmt.query_named(named_params!{
            ":id": id
        })?);
        Ok(result.next().unwrap().unwrap())
    }
    
    pub fn get_by_post_id(conn : Connection, post_id : i64) -> Result<Vec<Attachment>> {
        let mut stmt = conn.prepare_cached(
            &query_builder::select(Self::TABLE_NAME)
                .filter("post_id == :post_id")
                .build().to_string()
        )?;
        let query_result = from_rows::<Attachment>(stmt.query_named(named_params!{
            ":post_id": post_id
        })?);
        let mut result = Vec::<Attachment>::new();
        for attachment in query_result {
            result.push(attachment.unwrap());
        }
        Ok(result)
    }
    
    pub fn get_by_post(conn : Connection, post : Post) -> Result<Vec<Attachment>> {
        Self::get_by_post_id(conn, post.id)
    }
    
    pub fn save(conn : Connection, attachment : Attachment) -> Result<usize> {
        // This might not work as intended - it depends on how pinto builds the query
        conn.execute_named(
            &format!("INSERT OR REPLACE {}
                (id, post_id, filename)
                VALUES (:id, :post_id, :filename)", Self::TABLE_NAME),
            // &query_builder::insert(Self::TABLE_NAME)
            // .set("id", ":id")
            // .set("username", ":username")
            // .set("password", ":password")
            // .set("date_created", ":date_created")
            // .build().to_string(),
            &to_params_named(&attachment).unwrap().to_slice()
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
    
    pub fn delete(conn : Connection, user : Attachment) -> Result<usize> {
        Self::delete_by_id(conn, user.id)
    }
}
