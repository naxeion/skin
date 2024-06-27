use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

use crate::helper::get_target_data;
use crate::skinners::Metadata;
use crate::{error, error::throw};

pub fn r#do(target: &str) -> Option<Metadata> {
	let file_metadata_result = fs::metadata(target);
	if let Err(_err) = file_metadata_result {
		error::throw!(format!("File '{}' does not exist", target));
		return None;
	}
	let file_metadata = file_metadata_result.unwrap();

	let mut permissions = file_metadata.permissions();

	let mode = permissions.mode();
	let new_mode = mode & !(0o111);

	permissions.set_mode(new_mode);

	let _success: Result<(), Box<dyn std::error::Error>> =
		match fs::set_permissions(target, permissions) {
			Ok(_) => Ok(()),
			Err(err) => {
				error::throw!(format!("Error setting permissions: {}", err));
				return None;
			}
		};

	Some(Metadata {
		target,
		replaced_with: None,
		skinner: "X",
		status: true,
		last_use_date: None,
		last_use_cmd: None,
		created_at: None,
	})
}

pub fn undo(metadata: Metadata) -> bool {
	let file_metadata_result = fs::metadata(metadata.target);
	if let Err(_err) = &file_metadata_result {
		error::throw!(format!("File '{}' does not exist", metadata.target));
	}
	let file_metadata = file_metadata_result.unwrap();

	let mut permissions = file_metadata.permissions();

	let mode = permissions.mode();
	let new_mode = mode | 0o111;

	permissions.set_mode(new_mode);

	let set_permissions_result = fs::set_permissions(metadata.target, permissions);

	match set_permissions_result {
		Ok(_) => true,
		Err(ref err) => {
			error::throw!(format!("Error setting permissions: {}", err));
			false
		}
	}
}

pub fn action_run(mut target: &str, extras: &str) -> Result<bool, Box<dyn std::error::Error>> {
	if let Ok(metadata_vec) = get_target_data(target) {
		if let Some(target_metadata) = metadata_vec.first() {
			let args: Vec<&str> = extras.split_whitespace().collect();
			let args_to_pass: Vec<&OsStr> = args.iter().map(OsStr::new).collect();
			target = target_metadata.target;

			undo(target_metadata.clone());
			let mut cmd = Command::new(target)
				.args(&args_to_pass)
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.spawn()
				.expect("Failed to execute process");

			let status = cmd.wait().expect("Failed to wait on command process");
			r#do(target);

			return Ok(status.success());
		} else {
			throw!(error::TARGET_METADATA_EMPTY);
		}
	}

	Ok(false)
}
