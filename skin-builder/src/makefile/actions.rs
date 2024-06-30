use std::process::{Command, Output};

use crate::error::{self, throw};
use crate::makefile::get_skinner_file;

pub fn execute_makefile(file: &str, target: &str, action: &str) -> String {
	// Execute the skinner makefile
	let output: Output = Command::new("make")
		.args(["-f", file, action, &format!("TARGET={}", target)])
		.output()
		.expect("Failed to execute make command");

	// Convert the output to string
	let stdout = String::from_utf8_lossy(&output.stdout).to_string();
	let stderr = String::from_utf8_lossy(&output.stderr).to_string();

	// Return the output
	format!("{}\n{}", stdout, stderr)
}

pub fn r#do(target: &str, skinner: &str) -> (bool, Option<String>) {
	// Get the skinner file
	let skinner_file = get_skinner_file(skinner);

	// Check if the file exists
	if std::fs::metadata(&skinner_file).is_err() {
		throw!(error::SKINNER_FILE_NOT_FOUND, exit);
	}

	// Execute the make command
	let output = execute_makefile(&skinner_file, target, "do");

	// If the output contains "REPLACED_WITH: ", get the replaced with
	let mut replaced_with = None;
	if output.contains("REPLACED_WITH: ") {
		replaced_with = Some(
			output
				.split("REPLACED_WITH: ")
				.nth(1)
				.unwrap()
				.trim()
				.to_string(),
		);
	}

	// Check if the make command was successful
	(output.contains("do: success"), replaced_with)
}

pub fn undo(target: &str, skinner: &str) -> bool {
	// Get the skinner file
	let skinner_file = get_skinner_file(skinner);

	// Execute the make command
	let output = execute_makefile(&skinner_file, target, "undo");

	// Check if the make command was successful
	output.contains("undo: success")
}
