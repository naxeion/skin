use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

use skin_builder::installer;

#[derive(Parser)]
#[command(name = "skin-builder", version = "1.1.0")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Install { url: String },
}

pub fn execute_command(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
	match cli.command {
		Commands::Install { url } => {
			let rt = Runtime::new()?;
			rt.block_on(async {
				if let Ok(success) = installer::operate(&url).await {
					if success {
						color_print::cprintln!(
							"The skinner has been installed <green>successfully</green>."
						);
					}
				} else if let Err(err) = installer::operate(&url).await {
					eprintln!("Error installing file: {}", err);
				}
			});
		}
	};

	Ok(())
}
