use clap::Parser;
mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// println!("{:?}", validator::check_targets("./skinner/X/Makefile"));

	// if !as_root() {
	// 	color_print::cprintln!("<red,bold>Error:</> {}", error::ERROR_NOT_ROOT);
	// 	std::process::exit(0);
	// }

	// let _ = db::setup_database();
	// let _ = command::setup_config_dir(None);

	// check SKIN_SKINNERS_DIRECTORY if exist

	let cli = cli::Cli::parse();
	cli::execute_command(cli)
}
