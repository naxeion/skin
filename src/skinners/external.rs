use skin_builder::makefile;
use skin_builder::makefile::actions;
use skin_builder::makefile::get_skinner_file;
use std::process::{Command, Stdio};

use crate::error::{self, throw};
use crate::helper::get_target_data;
use crate::skinners::Metadata;

pub fn r#do<'a>(target: &'a str, skinner: &'a str) -> Option<Metadata<'a>> {
	if makefile::is_skinner_exist(skinner) {
		let (success, replaced_with) = actions::r#do(target, skinner);

		if success {
			let metadata = Metadata {
				target,
				replaced_with,
				skinner,
				status: true,
				last_use_date: None,
				last_use_cmd: None,
				created_at: None,
			};

			Some(metadata)
		} else {
			throw!(error::SKINNER_APPLY_FAILED, exit);
		}
	} else {
		throw!(error::SKINNER_NOT_EXIST, exit);
	}
}

pub fn undo(metadata: Metadata) -> bool {
	actions::undo(&metadata.target, &metadata.skinner)
}

pub fn action_run(
	mut target: &str,
	extras: &str,
	skinner: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
	if let Ok(metadata_vec) = get_target_data(target) {
		if let Some(target_metadata) = metadata_vec.first() {
			let skinner_file = get_skinner_file(skinner);
			let args: Vec<&str> = extras.split_whitespace().collect();
			target = target_metadata.target;

			let mut cmd = Command::new("make")
				.args(&[
					"-f",
					&skinner_file,
					"run",
					&format!("TARGET={}", target),
					&format!("ARGS={}", args.join(" ")),
				])
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.spawn()
				.expect("Failed to execute process");

			let status = cmd.wait().expect("Failed to wait on command process");

			return Ok(status.success());
		} else {
			throw!(error::TARGET_METADATA_EMPTY);
		}
	}

	Ok(false)
}
