use std::fs;
use std::io;
use std::process::Command;
use std::str;

use crate::config::SKIN_CONFIG_DIRECTORY;

pub fn find_command_paths(command: &str) -> Result<Vec<String>, io::Error> {
	let output = Command::new("which").arg("-a").arg(command).output();

	match output {
		Ok(output) => {
			if output.status.success() {
				let stdout = str::from_utf8(&output.stdout).expect("Failed to parse output");
				let mut vec: Vec<_> = stdout.lines().map(|line| line.to_string()).collect();

				if vec.iter().any(|element| element.starts_with("/bin")) {
					vec.retain(|item| !item.starts_with("/usr/bin"));
				}
				if vec.iter().any(|element| element.starts_with("/sbin")) {
					vec.retain(|item| !item.starts_with("/usr/sbin"));
				}

				Ok(vec)
			} else {
				Ok(Vec::new())
			}
		}
		Err(error) => Err(error),
	}
}

pub fn setup_config_dir() -> Result<(), std::io::Error> {
	let expanded_path = shellexpand::tilde(SKIN_CONFIG_DIRECTORY).into_owned();

	if !fs::metadata(&expanded_path).is_ok() {
		fs::create_dir_all(&expanded_path)?;
	}

	Ok(())
}
