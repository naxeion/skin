use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

use crate::config::SKIN_CONFIG_DIRECTORY;

pub static DATABASE_PATH: Lazy<String> =
	Lazy::new(|| format!("{}/main.db", SKIN_CONFIG_DIRECTORY.to_string()));

pub static DATABASE_SCHEMA: Lazy<String> = Lazy::new(|| {
	let current_file = file!();
	let current_dir = Path::new(current_file)
		.parent()
		.expect("Failed to get parent directory of current file");
	let schema_path = current_dir.join("db_modules/schema.sql");
	schema_path.to_string_lossy().into_owned()
});

pub fn connect_to_db(db_path: &str) -> Result<Connection> {
	Connection::open(db_path)
}

pub fn connect_to_main_db() -> Result<Connection> {
	connect_to_db(&*DATABASE_PATH)
}

pub fn execute_sql_from_file(
	conn: &rusqlite::Connection,
	file_path: &str,
) -> Result<(), rusqlite::Error> {
	let sql = fs::read_to_string(file_path);

	match sql {
		Ok(sql) => {
			conn.execute_batch(&sql)?;
		}
		Err(error) => {
			println!("{}", error);
		}
	}

	Ok(())
}

pub fn execute_sql(conn: &rusqlite::Connection, sql_code: &str) -> Result<()> {
	conn.execute_batch(&sql_code)?;
	Ok(())
}

pub fn setup_database() -> Result<()> {
	let conn = connect_to_main_db()?;
	execute_sql_from_file(&conn, &*DATABASE_SCHEMA)?;
	Ok(())
}
