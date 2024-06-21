use rskin::modules::command::*;

use std::fs;
use tempdir::TempDir;

#[test]
fn test_find_command_paths() {
	let command = "pwd";
	let result = find_command_paths(command);

	assert!(
		result.is_ok(),
		"Error executing find_command_paths: {:?}",
		result.err()
	);
	match result {
		Ok(paths) => {
			assert_ne!(paths, vec!["/bin/pwd", "/usr/bin/pwd"]);
			assert_eq!(paths, vec!["/bin/pwd"]);
		}
		Err(e) => {
			panic!("Error occurred: {:?}", e);
		}
	}
}

#[test]
fn test_get_alias_existing_alias() {
	let alias_command = "ls";
	let alias_value = get_alias(alias_command);
	assert!(alias_value.is_some());
}

#[test]
fn test_get_alias_non_existing_alias() {
	let alias_command = "nonexistent_command";
	let alias_value = get_alias(alias_command);
	assert!(alias_value.is_none());
}

#[test]
fn test_alias() {
	let command = "test_command";
	let alias_string = "echo Test alias";
	let success = alias(command, alias_string);
	assert!(success);

	// Check if the alias was added correctly
	let alias_value = get_alias(command);
	assert_eq!(alias_value, Some(String::from("echo Test alias")));
}

#[test]
fn test_unalias_existing_alias() {
	let command = "test_command_to_unalias";
	let alias_string = "echo 'Test alias to unalias'";
	alias(command, alias_string);

	let success = unalias(command);
	assert!(success);

	// Check if the alias was removed correctly
	let alias_value = get_alias(command);
	assert!(alias_value.is_none());
}

#[test]
fn test_unalias_non_existing_alias() {
	let command = "nonexistent_command_to_unalias";
	let success = unalias(command);

	assert!(!success);
}

#[test]
fn test_is_builtin_command() {
	assert!(is_builtin_command("pwd"));
	assert!(is_builtin_command("/bin/pwd"));
	assert!(is_builtin_command("/sbin/pwd"));

	assert!(is_builtin_command("cd"));

	assert!(!is_builtin_command("/usr/bin/sum"));
	assert!(!is_builtin_command("/usr/bin/crunch"));
	assert!(!is_builtin_command("nonexistentcommand"));
}

#[test]
fn test_setup_config_dir() {
	// Create a temporary directory and define the testing path
	let temp_dir = TempDir::new("skin_test").unwrap();
	let temp_path = temp_dir.path().to_str().unwrap();
	let test_path = format!("{}/.skin", temp_path);

	// Call the function with the temporary directory
	setup_config_dir(Some(&test_path)).unwrap();

	// Check if the directory was created
	assert!(fs::metadata(test_path).is_ok());

	// TempDir is automatically cleaned up, no need for manual cleanup
}
