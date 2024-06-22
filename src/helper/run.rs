use chrono::Utc;
use std::ffi::OsStr;
use std::process::{Command, Stdio};

use crate::helper::get_target_data;
use crate::modules::db;
use crate::{error, error::throw};

pub fn action(target: &str, extras: &str) -> Result<(), Box<dyn std::error::Error>> {
	let db_conn = db::connect_to_main_db()?;

	if let Ok(metadata_vec) = get_target_data(target) {
		if let Some(target_metadata) = metadata_vec.first() {
			let args: Vec<&str> = extras.split_whitespace().collect();
			let args_to_pass: Vec<&OsStr> = args.iter().map(OsStr::new).collect();
			let replaced_with = target_metadata.replaced_with.clone();

			let mut cmd = Command::new(replaced_with.clone())
				.args(&args_to_pass)
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.spawn()
				.expect("Failed to execute process");

			let status = cmd.wait().expect("Failed to wait on command process");

			if status.success() {
				let used_command = format!("{} {}", replaced_with, extras);
				let query = format!("UPDATE TARGETS SET LAST_USE_DATE='{date}', LAST_USE_CMD='{cmd}' WHERE REPLACED_WITH='{replaced_with}';",
						date = Utc::now(),
						cmd = used_command,
						replaced_with = replaced_with
					);

				db::execute_sql(&db_conn, &query)?;
			}
		} else {
			throw!(error::TARGET_METADATA_EMPTY)
		}
	}
	Ok(())
}
