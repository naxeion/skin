use clap::Parser;

use rskin::as_root;
use rskin::error;
use rskin::modules::command;
use rskin::modules::db;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	if !as_root() {
		color_print::cprintln!("<red,bold>Error:</> {}", error::ERROR_NOT_ROOT);
		std::process::exit(0);
	}

	let _ = db::setup_database();
	let _ = command::setup_config_dir(None);

	let cli = cli::Cli::parse();
	cli::execute_command(cli)
}
