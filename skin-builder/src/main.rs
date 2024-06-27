use clap::Parser;
mod cli;

use skin_builder::as_root;
use skin_builder::error;
use skin_builder::setup_config_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	if !as_root() {
		color_print::cprintln!("<red,bold>Error:</> {}", error::ERROR_NOT_ROOT);
		std::process::exit(0);
	}

	let _ = setup_config_dir(None);

	let cli = cli::Cli::parse();
	cli::execute_command(cli)
}
