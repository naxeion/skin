use chrono::Utc;

use crate::helper::get_target_data;
use crate::modules::db;
use crate::skinners::run_by_skinner;
use crate::{error, error::throw};

pub fn action(target: &str, extras: &str) -> Result<bool, Box<dyn std::error::Error>> {
	let db_conn = db::connect_to_main_db()?;

	if let Ok(metadata_vec) = get_target_data(target) {
		if let Some(target_metadata) = metadata_vec.first() {
			let replaced_with = target_metadata.replaced_with.clone();
			let skinner = target_metadata.skinner;

			let success = run_by_skinner(target, skinner, Some(extras));
			match success {
				Ok(success) => {
					if success {
						let used_command = format!(
							"{} {}",
							replaced_with.unwrap_or(target.to_string()).clone(),
							extras
						);
						let query = format!("UPDATE TARGETS SET LAST_USE_DATE='{date}', LAST_USE_CMD='{cmd}' WHERE TARGET='{target}';",
								date = Utc::now(),
								cmd = used_command,
								target = target_metadata.target
							);

						db::execute_sql(&db_conn, &query)?;
					}
				}
				Err(_) => todo!(),
			}

			return success;
		} else {
			throw!(error::TARGET_METADATA_EMPTY)
		}
	}

	Ok(false)
}
