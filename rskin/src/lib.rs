extern crate clean_path;
extern crate libc;

use rand::{distributions::Alphanumeric, Rng};
use std::path::PathBuf;

pub mod config;
pub mod error;
pub mod helper;
pub mod modules;
pub mod skinners;

pub fn get_parent_directory(file_path: &str) -> String {
	let mut path_buf = PathBuf::from(file_path);
	path_buf.pop();
	path_buf.to_string_lossy().into_owned()
}

pub fn clean_path(path: &str) -> String {
	let cleaned_path = clean_path::clean(path);
	cleaned_path.to_string_lossy().into_owned()
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
