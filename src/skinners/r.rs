use rand::Rng;
use std::fs;

use crate::skinners::Metadata;
use crate::{get_parent_directory, get_random_string};

fn get_a_hide_name() -> String {
	let mut rng = rand::thread_rng();
	let number = rng.gen_range(10..16);

	get_random_string(number)
}

pub fn r#do(target: &str) -> Option<Metadata> {
	let s: String = get_a_hide_name();

	if let Ok(file_metadata) = fs::metadata(target) {
		if file_metadata.is_file() {
			let path = get_parent_directory(target);
			let replaced_with: String = format!("{}/{}", path, s);

			let _ = fs::rename(target, replaced_with.clone());
			Some(Metadata {
				target,
				replaced_with,
				skinner: "R",
				status: true,
				last_use_date: None,
				last_use_cmd: None,
				created_at: None,
			})
		} else {
			panic!("Target '{}' exists, but it's not a file.", target);
		}
	} else {
		panic!("File '{}' does not exist.", target);
	}
}

pub fn undo(metadata: Metadata) {
	if let Ok(file_metadata) = fs::metadata(&metadata.replaced_with) {
		if file_metadata.is_file() {
			let _ = fs::rename(metadata.replaced_with, metadata.target);
		}
	} else {
		println!("The replacement file for the target does not exist");
	}
}
