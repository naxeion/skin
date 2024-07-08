use dialoguer::Confirm;
use indicatif::ProgressBar;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use url::Url;

use crate::config::{SKIN_SKINNERS_DIRECTORY, SKIN_TEMP_DIRECTORY};
use crate::error::throw;
use crate::makefile::get_skinner_name;
use crate::validator;

pub async fn operate(url: &str) -> Result<bool, Box<dyn std::error::Error>> {
	#[allow(unused_assignments)]
	let mut filepath: String = String::new();

	if is_path(url) {
		filepath = url.to_string();
	} else if is_url(url) {
		if let Some(url_filename) = get_filename_from_url(url) {
			filepath = format!("{}/{}", SKIN_TEMP_DIRECTORY, &url_filename);
			download_file(url, &filepath).await?;
		} else {
			throw!("No filename found in the URL.", exit);
		}
	} else {
		throw!("Invalid URL or path.", exit);
	}

	if !filepath.is_empty() {
		if validator::check_targets(&filepath).is_empty() {
			if validator::check_variables(&filepath).is_empty() {
				if validator::is_valid_name(&get_skinner_name(&filepath)) {
					return Ok(apply(&filepath));
				} else {
					throw!(
						format!("Invalid skinner name: {}", get_skinner_name(&filepath)),
						exit
					);
				}
			} else {
				throw!("Some doc variables does not exist.", exit);
			}
		} else {
			throw!("Some main targets does not exist.", exit);
		}
	}

	Ok(false)
}

pub fn apply(filename: &str) -> bool {
	if !validator::is_valid_name(&get_skinner_name(filename)) {
		throw!(
			format!("Invalid skinner name: {}", get_skinner_name(filename)),
			exit
		);
	}

	let src = Path::new(filename);
	let dest =
		Path::new(SKIN_SKINNERS_DIRECTORY).join(format!("{}.makefile", get_skinner_name(filename)));

	fs::copy(src, dest).is_ok()
}

pub async fn download_file(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut response = reqwest::get(url).await?;

	if Path::new(filename).exists() {
		let confirm_overwrite = Confirm::new()
			.with_prompt("The skinner is exist already, do you want to overwrite it?")
			.default(false)
			.show_default(true)
			.interact()?;

		if confirm_overwrite {
			fs::remove_file(filename)?;
			// color_print::cprintln!(
			// 	"<yellow>The current targets will be converted to the new skinner.</>"
			// );

			// let confirm_continue = Confirm::new()
			// 	.with_prompt("Are you sure you want to continue?")
			// 	.default(false)
			// 	.show_default(true)
			// 	.interact()?;

			// if confirm_continue {
			// 	fs::remove_file(filename)?;
			// } else {
			// 	throw!("Skinner already exists and overwrite not confirmed.", exit);
			// }
		} else {
			throw!("Skinner already exists and overwrite not confirmed.", exit);
		}
	}

	let length = response.content_length().unwrap_or(0);
	let pb = ProgressBar::new(length);
	pb.set_style(
		indicatif::ProgressStyle::default_bar()
			.template("[{elapsed_precise}] {bar:30.black/white} {bytes}/{total_bytes} ({eta})")?
			.progress_chars("- -"),
	);

	if response.status().is_success() {
		let mut file = File::create(filename)?;

		let mut downloaded = 0;
		while let Some(chunk) = response.chunk().await? {
			file.write_all(&chunk)?;
			downloaded += chunk.len() as u64;
			pb.set_position(downloaded);
		}

		pb.finish_with_message("Download complete");

		color_print::cprintln!("The skinner has been downloaded <green>successfully</>.");
	} else {
		throw!("Failed to download file.", exit);
	}

	Ok(())
}

pub fn get_filename_from_url(url: &str) -> Option<String> {
	if let Ok(parsed_url) = Url::parse(url) {
		if let Some(segments) = parsed_url.path_segments() {
			if let Some(filename) = segments.last() {
				return Some(filename.to_string());
			}
		}
	}

	None
}

pub fn is_url(input: &str) -> bool {
	Url::parse(input).is_ok()
}

pub fn is_path(input: &str) -> bool {
	Path::new(input).exists()
}
