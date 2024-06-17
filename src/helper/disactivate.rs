use crate::modules::command;
use crate::modules::db;
use crate::skinners::r;

pub fn action(target: &str) -> Result<(), Box<dyn std::error::Error>> {
	let paths = command::find_command_paths(target);
	let db_conn = db::connect_to_main_db()?;

	match paths {
		Ok(ref paths) => {
			if paths.is_empty() {
				println!(
					"The command `{}` does not exist or may have been deactivated previously",
					target
				);
			}
			for item in paths {
				if let Some(skinner_metadata) = r::r#do(item) {
					let query = format!("INSERT INTO TARGETS (TARGET, REPLACED_WITH, SKINNER, STATUS) VALUES ('{}', '{}', '{}', TRUE)", 
                        skinner_metadata.target,
                        skinner_metadata.replaced_with,
                        skinner_metadata.skinner,
                    );

					db::execute_sql(&db_conn, &query)?;
				}
			}
		}
		Err(error) => panic!("Error searching for command paths: {:?}", error),
	}

	Ok(())
}
