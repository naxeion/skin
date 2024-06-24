use rand::Rng;
use std::ffi::OsStr;
use std::fs;
use std::process::{Command, Stdio};

use crate::clean_path;
use crate::helper::get_target_data;
use crate::modules::command;
use crate::skinners::Metadata;
use crate::{error, error::throw};
use crate::{get_parent_directory, get_random_string};

fn get_a_hide_name() -> String {
	let mut rng = rand::thread_rng();
	let number = rng.gen_range(10..16);

	get_random_string(number)
}

pub fn r#do(target: &str) -> Option<Metadata> {
	if let Ok(file_metadata) = fs::metadata(target) {
		if file_metadata.is_file() {
			let path = get_parent_directory(target);
			let mut replaced_with: String;

			loop {
				replaced_with = clean_path(format!("{}/{}", path, get_a_hide_name()).as_str());

				if !command::file_exists(&replaced_with) {
					break;
				}
			}

			let _ = fs::rename(target, replaced_with.clone());
			Some(Metadata {
				target,
				replaced_with: Some(replaced_with),
				skinner: "R",
				status: true,
				last_use_date: None,
				last_use_cmd: None,
				created_at: None,
			})
		} else {
			panic!("Target '{}' exists, but it's not a file.", target);
		}
	} else {
		panic!("File '{}' does not exist.", target);
	}
}

pub fn undo(metadata: Metadata) -> bool {
	if let Some(replaced_with) = metadata.replaced_with {
		if let Ok(file_metadata) = fs::metadata(&replaced_with) {
			if file_metadata.is_file() {
				let _ = fs::rename(replaced_with, metadata.target);
				return true;
			}
		} else {
			throw!(error::REPLACEMENT_FILE_NOT_FOUND);
		}
	} else {
		throw!(error::REPLACEMENT_FILE_NOT_FOUND);
	}

	false
}

pub fn action_run(target: &str, extras: &str) -> Result<bool, Box<dyn std::error::Error>> {
	if let Ok(metadata_vec) = get_target_data(target) {
		if let Some(target_metadata) = metadata_vec.first() {
			let args: Vec<&str> = extras.split_whitespace().collect();
			let args_to_pass: Vec<&OsStr> = args.iter().map(OsStr::new).collect();

			if let Some(replaced_with) = target_metadata.replaced_with.clone() {
				let mut cmd = Command::new(replaced_with.clone())
					.args(&args_to_pass)
					.stdin(Stdio::inherit())
					.stdout(Stdio::inherit())
					.stderr(Stdio::inherit())
					.spawn()
					.expect("Failed to execute process");

				let status = cmd.wait().expect("Failed to wait on command process");

				return Ok(status.success());
			} else {
				throw!(error::REPLACEMENT_FILE_NOT_FOUND);
			}
		} else {
			throw!(error::TARGET_METADATA_EMPTY);
		}
	}

	Ok(false)
}
