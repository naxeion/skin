use skin_builder::makefile;

use crate::error::{self, throw};
use crate::modules::command;
use crate::modules::db;
use crate::skinners::is_skinner_exist;
use crate::skinners::r#do;

pub fn action(target: &str, skinner: &str) -> Result<bool, Box<dyn std::error::Error>> {
	let paths = command::find_command_paths(target);
	let db_conn = db::connect_to_main_db()?;

	if command::is_builtin_command(target) {
		throw!(error::CANNOT_DISACTIVATE_BUILTIN, exit);
	}

	if is_skinner_exist(skinner) || makefile::is_skinner_exist(skinner) {
		match paths {
			Ok(paths) => {
				if paths.is_empty() {
					throw!(error::TARGET_UNDEACTIVATABLE);
					return Ok(false);
				}
				for item in paths {
					if let Some(skinner_metadata) = r#do(&item, skinner) {
						let query = format!("INSERT INTO TARGETS (TARGET, REPLACED_WITH, SKINNER, STATUS) VALUES ('{}', {}, '{}', TRUE)", 
							skinner_metadata.target,
							skinner_metadata.replaced_with.as_ref().map_or("Null".to_string(), |s| format!("'{}'", s)),
							skinner_metadata.skinner,
						);

						db::execute_sql(&db_conn, &query)?;
					}
				}

				color_print::cprintln!("The target has been disactivated <green>successfully</>.");

				return Ok(true);
			}
			Err(error) => throw!(
				format!("Error searching for command paths: {:?}", error),
				exit
			),
		}
	} else {
		throw!(error::SKINNER_NOT_EXIST);
	}

	Ok(false)
}
