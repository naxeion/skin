use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

use crate::config::SKIN_CONFIG_DIRECTORY;

// Path to the main database file
pub static DATABASE_PATH: Lazy<String> =
	Lazy::new(|| format!("{}/main.db", SKIN_CONFIG_DIRECTORY.to_string()));

// Path to the SQL schema file for database initialization
pub static DATABASE_SCHEMA: Lazy<String> = Lazy::new(|| {
	// Determine the directory of the current file to locate the schema file
	let current_file = file!();
	let current_dir = Path::new(current_file)
		.parent()
		.expect("Failed to get parent directory of current file");
	let schema_path = current_dir.join("db_modules/schema.sql");
	schema_path.to_string_lossy().into_owned()
});

// Connects to a SQLite database at the specified path.
pub fn connect_to_db(db_path: &str) -> Result<Connection> {
	Connection::open(db_path)
}

// Connects to the main SQLite database specified in DATABASE_PATH.
pub fn connect_to_main_db() -> Result<Connection> {
	connect_to_db(&*DATABASE_PATH)
}

// Executes SQL commands read from a file against the provided database connection.
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

// Executes a single SQL command against the provided database connection.
pub fn execute_sql(conn: &rusqlite::Connection, sql_code: &str) -> Result<()> {
	conn.execute_batch(&sql_code)?;
	Ok(())
}

// Sets up the main database by executing the schema SQL from DATABASE_SCHEMA
pub fn setup_database() -> Result<()> {
	let conn = connect_to_main_db()?;
	execute_sql_from_file(&conn, &*DATABASE_SCHEMA)?;
	Ok(())
}
