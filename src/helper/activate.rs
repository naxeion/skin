use crate::helper::get_target_data;
use crate::modules::db;
use crate::skinners::r;

pub fn action(target: &str) -> Result<(), Box<dyn std::error::Error>> {
	match get_target_data(target) {
		Ok(metadata_vec) => {
			let count = metadata_vec.len();
			if count == 0 {
				println!("The command does not exist in the database, or it is activated");
			} else {
				let db_conn = db::connect_to_main_db().expect("Failed to connect to the database");
				for metadata in metadata_vec {
					match db::execute_sql(
						&db_conn,
						format!("DELETE FROM TARGETS WHERE TARGET = '{}'", metadata.target)
							.as_str(),
					) {
						Ok(_) => {
							r::undo(metadata);
						}
						Err(err) => println!("Error deleting from database: {:?}", err),
					}
				}
			}
		}
		Err(err) => println!("Error retrieving target data: {:?}", err),
	}

	Ok(())
}
