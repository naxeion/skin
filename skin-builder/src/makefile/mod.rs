pub mod actions;

use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

use crate::config::SKIN_SKINNERS_DIRECTORY;
use crate::error::throw;

pub fn is_target_exist(filename: &str, target: &str) -> bool {
	// Check if the Makefile exists
	if !Path::new(filename).exists() {
		return false;
	}

	// Execute make command and capture its stderr
	let output = Command::new("make")
		.arg("-f")
		.arg(filename)
		.arg("-n")
		.arg(target)
		.stderr(Stdio::piped()) // Capture stderr
		.output();

	// Handle errors in executing make
	let output = match output {
		Ok(output) => output,
		Err(_) => {
			eprintln!("Failed to execute make command");
			return false;
		}
	};

	let stderr_output = String::from_utf8_lossy(&output.stderr);
	let stdout_output = String::from_utf8_lossy(&output.stdout);

	!(stderr_output.contains("No rule to make target")
		|| stderr_output.contains("Nothing to be done")
		|| stdout_output.contains("No rule to make target")
		|| stdout_output.contains("Nothing to be done"))
}

pub fn get_variable(filename: &str, name: &str) -> Option<String> {
	// Check if file exist
	let output = Command::new("make")
		.arg("-p")
		.arg("-f")
		.arg(filename)
		.output()
		.expect("Failed to execute command");

	if output.status.success() {
		let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

		for line in stdout.lines() {
			if line.trim().starts_with(&format!("{} =", name)) {
				let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
				if let Some(var_value) = parts.get(1) {
					let trimmed_line = var_value.trim();
					let start_index = trimmed_line.find('"').unwrap_or(trimmed_line.len());
					let end_index = trimmed_line.rfind('"').unwrap_or(start_index);
					let value = trimmed_line[start_index + 1..end_index].trim();
					return Some(value.to_string());
				}
			}
		}
	} else {
		throw!("Failed to execute command.", exit);
	}

	None
}

pub fn is_variable_exist(filename: &str, name: &str) -> bool {
	get_variable(filename, name).is_some()
}

pub fn get_skinner_name(filename: &str) -> String {
	if let Some(name) = get_skinner_name_from_filename(filename) {
		return name;
	}

	if is_variable_exist(filename, "name") {
		if let Some(name) = get_variable(filename, "name") {
			name
		} else {
			throw!("The variable Name does not contain a value", exit);
		}
	} else {
		throw!("Name variable not found", exit);
	}
}

pub fn get_skinner_name_from_filename(filename: &str) -> Option<String> {
	let path = Path::new(filename);
	let file_name = path.file_name()?.to_string_lossy().to_lowercase();

	if file_name.ends_with(".makefile") {
		let skinner_name = file_name[0..file_name.len() - 9].to_string(); // 9 is the length of ".makefile"
		Some(skinner_name)
	} else {
		None
	}
}

pub fn is_skinner_exist(name: &str) -> bool {
	let filepath = &format!("{SKIN_SKINNERS_DIRECTORY}/{}.makefile", name);
	let path = Path::new(filepath);
	path.exists()
}

pub fn get_skinner_file(name: &str) -> String {
	format!("{SKIN_SKINNERS_DIRECTORY}/{}.makefile", name)
}
