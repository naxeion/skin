use rskin::args;
use rskin::as_root;
use rskin::error;
use rskin::handle_subcommand;
use rskin::modules::command;
use rskin::modules::db;
use std::ffi::OsStr;
use std::process;

fn main() {
	if as_root() == false {
		color_print::cprintln!("<red,bold>Error:</> {}", error::ERROR_NOT_ROOT);
		process::exit(0);
	}

	let _ = db::setup_database();
	let _ = command::setup_config_dir();

	let (mut cmd, matches) = args::parse_args();

	if let Some((subcommand, sub_matches)) = matches.subcommand() {
		let mut extras_arg: String = "".to_string();
		match sub_matches.try_contains_id("extras") {
			Ok(status) => {
				if status {
					if let Some(mut extras) = sub_matches.get_raw("extras") {
						let first_arg = extras.next().map_or_else(|| OsStr::new(""), |arg| &arg);
						let rest_of_arguments: Vec<String> = extras
							.map(|arg| arg.to_string_lossy().to_string())
							.collect();

						let extras_as_string = first_arg.to_string_lossy().to_string();
						let rest_as_string = rest_of_arguments.join(" ");

						extras_arg = format!("{} {}", extras_as_string, rest_as_string);
					}
				}
			}
			Err(..) => {}
		}

		handle_subcommand(subcommand, sub_matches, &extras_arg);
	} else {
		let _ = cmd.print_help();
		println!();
	}
}
