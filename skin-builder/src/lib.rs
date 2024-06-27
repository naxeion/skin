extern crate libc;

pub mod config;
pub mod error;
pub mod installer;
pub mod makefile;
pub mod validator;

use crate::config::{SKIN_CONFIG_DIRECTORY, SKIN_SKINNERS_DIRECTORY, SKIN_TEMP_DIRECTORY};
use std::fs;

pub fn setup_config_dir(custom_path: Option<&str>) -> Result<(), std::io::Error> {
	for s_path in [
		SKIN_CONFIG_DIRECTORY,
		SKIN_SKINNERS_DIRECTORY,
		SKIN_TEMP_DIRECTORY,
	] {
		let path = custom_path.unwrap_or(s_path);
		let expanded_path = shellexpand::tilde(path).into_owned();

		if fs::metadata(&expanded_path).is_err() {
			fs::create_dir_all(&expanded_path)?;
		}
	}

	Ok(())
}

pub fn as_root() -> bool {
	let uid = unsafe { libc::getuid() };
	uid == 0
}
