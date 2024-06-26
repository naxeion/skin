use regex::Regex;

use crate::makefile;

pub const TARGETS: &[(&str, bool)] = &[
	// NAME | REQUIRED
	("do", true),
	("undo", true),
	("run", true),
	("status", true),
];

pub const VARIABLES: &[(&str, bool)] = &[
	// NAME | REQUIRED
	("name", false),
];

pub fn check_targets(makefile: &str) -> Vec<&str> {
	let mut not_exist = Vec::new();

	for &(name, _) in TARGETS {
		if !makefile::is_target_exist(makefile, name) {
			not_exist.push(name);
		}
	}

	not_exist
}

pub fn check_variables(makefile: &str) -> Vec<&str> {
	let mut not_exist = Vec::new();

	for &(name, required) in VARIABLES {
		if !makefile::is_variable_exist(makefile, name)
			&& (required || !check_not_r_existence(makefile, name))
		{
			not_exist.push(name);
		}
	}

	not_exist
}

pub fn is_valid_name(name: &str) -> bool {
	let re = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
	re.is_match(name)
}

pub fn check_not_r_existence(makefile: &str, name: &str) -> bool {
	match name {
		"name" => makefile::get_skinner_name_from_filename(makefile).is_some(),
		_ => false,
	}
}
