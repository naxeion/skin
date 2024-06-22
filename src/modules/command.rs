use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::str;

use crate::config::SKIN_CONFIG_DIRECTORY;

// List of some common Linux built-in commands
const BUILT_IN_COMMANDS: &[&str] = &[
	"alias",
	"bg",
	"bind",
	"break",
	"builtin",
	"cd",
	"command",
	"compgen",
	"complete",
	"compopt",
	"continue",
	"declare",
	"dirs",
	"disown",
	"echo",
	"enable",
	"eval",
	"exec",
	"exit",
	"export",
	"fc",
	"fg",
	"getopts",
	"hash",
	"help",
	"history",
	"jobs",
	"kill",
	"let",
	"local",
	"logout",
	"popd",
	"printf",
	"pushd",
	"pwd",
	"read",
	"readonly",
	"return",
	"set",
	"shift",
	"shopt",
	"source",
	"suspend",
	"test",
	"times",
	"trap",
	"type",
	"typeset",
	"ulimit",
	"umask",
	"unalias",
	"unset",
	"wait",
	"true",
	"false",
	"readarray",
	"mapfile",
	"shopt",
	"caller",
	"declare",
	"typeset",
	"readonly",
	"source",
	".",
	"shopt",
	"coproc",
	"caller",
	"readarray",
	"mapfile",
];

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

pub fn get_alias(command: &str) -> Option<String> {
	let output = Command::new("bash")
		.arg("-i")
		.arg("-c")
		.arg(format!("alias {}", command))
		.output()
		.expect("Failed to execute command");

	if output.status.success() {
		let output_str = String::from_utf8_lossy(&output.stdout).to_string();
		if output_str.trim().is_empty() {
			None
		} else {
			let parts: Vec<&str> = output_str.splitn(2, '=').collect();
			if parts.len() == 2 {
				let alias_value = parts[1].trim();
				Some(alias_value.trim_matches('\'').to_string())
			} else {
				None
			}
		}
	} else {
		None
	}
}

pub fn alias(command: &str, alias: &str) -> bool {
	let mut profile_path = match dirs::home_dir() {
		Some(path) => path.join(".bashrc"),
		None => return false,
	};

	if cfg!(target_os = "macos") {
		profile_path = profile_path.with_extension(".zshrc");
	}

	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(profile_path)
		.expect("Failed to open profile file");

	writeln!(file, "alias {}='{}'\n", command, alias).expect("Failed to write to profile");
	true
}

pub fn unalias(command: &str) -> bool {
	let alias_value = get_alias(command);
	if alias_value.is_none() {
		return false;
	}

	let mut profile_path = match dirs::home_dir() {
		Some(path) => path.join(".bashrc"),
		None => return false,
	};

	if cfg!(target_os = "macos") {
		profile_path = profile_path.with_extension(".zshrc");
	}

	let mut file_content = String::new();
	let mut file = std::fs::File::open(profile_path.clone()).expect("Failed to open profile file");
	file.read_to_string(&mut file_content)
		.expect("Failed to read profile file");

	let new_content = file_content
		.lines()
		.filter(|line| !line.starts_with(&format!("alias {}=", command)))
		.collect::<Vec<_>>()
		.join("\n");

	let mut file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(profile_path.clone())
		.expect("Failed to open profile file");

	write!(file, "{}", new_content).expect("Failed to write to profile");
	true
}

pub fn is_builtin_command(command: &str) -> bool {
	let command = if let Some(pos) = command.find("/bin/") {
		&command[pos + 5..]
	} else if let Some(pos) = command.find("/sbin/") {
		&command[pos + 6..]
	} else {
		command
	};

	BUILT_IN_COMMANDS.contains(&command)
}

pub fn setup_config_dir(custom_path: Option<&str>) -> Result<(), std::io::Error> {
	let path = custom_path.unwrap_or(SKIN_CONFIG_DIRECTORY);
	let expanded_path = shellexpand::tilde(path).into_owned();

	if fs::metadata(&expanded_path).is_err() {
		fs::create_dir_all(&expanded_path)?;
	}

	Ok(())
}

pub fn get_profile_extension() -> &'static str {
	if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
		".zshrc"
	} else if cfg!(target_os = "linux") {
		if is_bash() {
			".bashrc"
		} else if is_zsh() {
			".zshrc"
		} else {
			".profile"
		}
	} else if cfg!(target_os = "windows") {
		"autoexec.bat"
	} else {
		".profile"
	}
}

pub fn is_bash() -> bool {
	std::env::var("SHELL").unwrap_or_default().contains("bash")
}

pub fn is_zsh() -> bool {
	std::env::var("SHELL").unwrap_or_default().contains("zsh")
}
