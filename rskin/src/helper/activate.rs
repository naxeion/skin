use crate::helper::get_target_data;
use crate::modules::db;
use crate::skinners::undo;
use crate::{error, error::throw};

pub fn action(target: &str) -> Result<bool, Box<dyn std::error::Error>> {
	match get_target_data(target) {
		Ok(metadata_vec) => {
			let count = metadata_vec.len();
			if count == 0 {
				throw!(error::TARGET_UNACTIVATABLE);
			} else {
				let db_conn = db::connect_to_main_db().expect("Failed to connect to the database");
				for metadata in metadata_vec {
					if undo(metadata.clone()) {
						match db::execute_sql(
							&db_conn,
							format!("DELETE FROM TARGETS WHERE TARGET = '{}'", metadata.target)
								.as_str(),
						) {
							Ok(_) => {}
							Err(err) => println!("Error deleting from database: {:?}", err),
						}
					}
				}

				color_print::cprintln!("The target has been activated <green>successfully</>.");
			}
		}
		Err(err) => println!("Error retrieving target data: {:?}", err),
	}

	Ok(false)
}
