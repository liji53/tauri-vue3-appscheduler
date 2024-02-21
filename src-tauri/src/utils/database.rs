use super::program_db_path;
use rusqlite::{Connection, Result};

pub fn db_session(error: Option<&str>) -> Result<Connection, String> {
    match error {
        Some(error) => {
            Connection::open(program_db_path()).map_err(|_| format!("{}, 数据库链接异常", error))
        }
        None => Connection::open(program_db_path()).map_err(|_| "数据库链接异常".to_string()),
    }
}
