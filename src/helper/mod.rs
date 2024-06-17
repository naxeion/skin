use rusqlite::{params, Result};

use crate::modules::db;
use crate::skinners::Metadata;

pub mod activate;
pub mod disactivate;
pub mod run;
pub mod status;

pub fn get_target_data(target: &str) -> Result<Vec<Metadata<'static>>> {
	let db_conn = db::connect_to_main_db()?;

	let sql = "SELECT TARGET, REPLACED_WITH, SKINNER, STATUS, LAST_USE_DATE, LAST_USE_CMD, CREATED_AT FROM TARGETS WHERE TARGET LIKE ?1";
	let mut stmt = db_conn.prepare(sql)?;

	let pattern = if target.contains("/") {
		format!("%{}", target)
	} else {
		format!("%/{}", target)
	};

	let metadata_iter = stmt.query_map(params![pattern], |row| {
		let target: String = row.get(0)?;
		let replaced_with: String = row.get(1)?;
		let skinner: String = row.get(2)?;
		let status: bool = row.get(3)?;

		let last_use_date: Option<String> = row.get(4)?;
		let last_use_cmd: Option<String> = row.get(5)?;
		let created_at: Option<String> = row.get(6)?;

		let target_ref: &'static str = Box::leak(target.into_boxed_str());
		let skinner_ref: &'static str = Box::leak(skinner.into_boxed_str());

		Ok(Metadata {
			target: target_ref,
			replaced_with,
			skinner: skinner_ref,
			status,
			last_use_date: Some(last_use_date.unwrap_or(String::new())),
			last_use_cmd: Some(last_use_cmd.unwrap_or(String::new())),
			created_at: Some(created_at.unwrap_or(String::new())),
		})
	})?;

	let metadata_vec: Result<Vec<_>, _> = metadata_iter.collect();
	metadata_vec
}
