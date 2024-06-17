extern crate libc;
use rand::{distributions::Alphanumeric, Rng};
use std::path::PathBuf;

pub mod args;
pub mod commands;
pub mod config;
pub mod error;
pub mod helper;
pub mod modules;
pub mod skinners;

pub fn handle_subcommand(subcommand: &str, matches: &clap::ArgMatches, extras: &str) {
	match subcommand {
		"db" => {}
		"run" => {
			let target = matches.get_one::<String>("target").unwrap();
			commands::run_command(target, extras);
		}
		"activate" | "disactivate" | "status" => {
			let target = matches.get_one::<String>("target").unwrap();
			commands::bar_command(subcommand, target);
		}
		_ => {
			println!("Unknown command: {}", subcommand);
		}
	}
}

pub fn get_parent_directory(file_path: &str) -> String {
	let mut path_buf = PathBuf::from(file_path);
	path_buf.pop();
	path_buf.to_string_lossy().into_owned()
}

pub fn get_random_string(length: usize) -> String {
	rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(length)
		.map(char::from)
		.collect()
}

pub fn as_root() -> bool {
	let uid = unsafe { libc::getuid() };
	uid == 0
}
