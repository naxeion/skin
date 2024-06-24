use clap::{Parser, Subcommand};

use rskin::helper::{activate, disactivate, run, status};

#[derive(Parser)]
#[command(
	name = "skin",
	version = "1.3.0",
	about = "A project for managing commands and applying skinners"
)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Activate {
		target: String,
	},
	Disactivate {
		target: String,
		skinner: Option<String>,
	},
	Status {
		target: String,
	},
	Run {
		target: String,
		#[arg(allow_hyphen_values = true, num_args = 1..)]
		extras: Vec<String>,
	},
	Config {
		command: String,
	},
}

pub fn execute_command(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
	let _ = match cli.command {
		Commands::Activate { target } => activate::action(&target),
		Commands::Disactivate { target, skinner } => {
			disactivate::action(&target, &skinner.unwrap_or("R".to_string()))
		}
		Commands::Status { target } => status::action(&target),
		Commands::Run { target, extras } => run::action(&target, &string_extras(&extras)),
		Commands::Config { command: _ } => todo!(),
	};

	Ok(())
}

pub fn string_extras(extras: &[String]) -> String {
	extras.join(" ")
}