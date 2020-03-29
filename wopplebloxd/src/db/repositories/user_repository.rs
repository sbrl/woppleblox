use rusqlite::{ Statement, Result };
use crate::db::definitions::{ Connection };
use pinto::query_builder;

pub enum QueryType {
    GetById(i64),
    GetByName(String)
}


// #[macro_export]
// macro_rules! prepare_sql {
//     ( $table:expr, $query:expr ) => {
//         {
//             conn.prepare(&format!($query, $table).to_string())
//         }
//     }
// }

pub struct UserRepository {
    statement_by_id : Statement
}

impl UserRepository {
    const TABLE_NAME : &'static str = "users";
    const COL_ID : &'static str = "id";
    const COL_USERNAME : &'static str = "username";
    const COL_PASSWORD : &'static str = "password";
    const COL_DATE_CREATED : &'static str = "date_created";
    
    fn new(conn : Connection) -> UserRepository {
        let result = UserRepository {
            statement_by_id: Self::query_get_by_id(conn).unwrap()
        };
        result
    }
    
    fn query_get_by_id(conn : Connection) -> Result<Statement> {
        conn.prepare(
            &query_builder::select(Self::TABLE_NAME)
                .filter("id == :id")
                .build().to_string()
            // &format!("SELECT * FROM {} WHERE ", TABLE_NAME).to_string())
        )
    }
    
    
    
    pub fn query(query_type : QueryType) {
        // match query_type {
        // 
        // }
    }
}
