use color_print::cprintln;

use crate::helper::get_target_data;
use crate::modules::command;
use crate::{error, error::throw};

pub fn action(target: &str) -> Result<(), Box<dyn std::error::Error>> {
	match get_activation_status(target) {
		Ok(activation_status) => {
			if activation_status == true {
				match command::find_command_paths(target) {
					Ok(command_paths) => {
						if command_paths.iter().count() == 0 {
							throw!(error::NOT_EXISTING_ENTITY, exit);
						} else {
							cprintln!("Active: <green,bold>{}</>", "active (usable)");
						}
					}
					Err(..) => {}
				}
			} else {
				cprintln!("Active: <red,bold>{}</>", "inactive (unusable)");
				match get_target_data(target) {
					Ok(metadata_vec) => {
						for (index, metadata) in metadata_vec.iter().enumerate() {
							cprintln!(
								"<red!,bold>Target</> {} <R!>[{}]</R!>: since {}",
								metadata.target,
								format!("/usr{}", metadata.target),
								metadata.created_at.as_deref().unwrap_or("Unknown")
							);
							println!("{:<4}- Skinner       : \"{}\"", "", metadata.skinner);
							println!("{:<4}- Replaced with : {}", "", metadata.replaced_with);

							if let Some(date) = metadata.last_use_date.as_deref() {
								if !date.is_empty() {
									println!("{:<4}- Last use time : {:?}", "", date);
								}
							}

							if let Some(cmd) = metadata.last_use_cmd.as_deref() {
								if !cmd.is_empty() {
									println!("{:<4}- Last use cmd  : {}", "", cmd);
								}
							}

							if index < metadata_vec.len() - 1 {
								println!();
							}
						}
					}
					Err(err) => println!("Error retrieving target data: {:?}", err),
				}
			}
		}
		Err(err) => println!("Error retrieving target data: {:?}", err),
	}

	Ok(())
}

pub fn get_activation_status(target: &str) -> Result<bool, Box<dyn std::error::Error>> {
	match get_target_data(target) {
		Ok(metadata_vec) => {
			if metadata_vec.len() == 0 {
				return Ok(true);
			}
		}
		Err(err) => println!("Error retrieving target data: {:?}", err),
	}
	Ok(false)
}
