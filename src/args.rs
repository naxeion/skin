use clap::{Arg, ArgMatches, Command};

// .subcommand(
//     Command::new("db")
//         .arg(Arg::new("target").index(1))
//         .arg(Arg::new("action").index(2)),
// )

pub fn parse_args() -> (Command, ArgMatches) {
	let cmd = Command::new("rskin")
		.version("1.2.0")
		.about("A project for managing commands and applying skinners")
		.subcommand(
			Command::new("run")
				.arg(Arg::new("target").required(true).index(1))
				.arg(
					Arg::new("extras")
						.index(2)
						.allow_hyphen_values(true)
						.last(true)
						.num_args(0..),
				),
		)
		.subcommand(Command::new("activate").arg(Arg::new("target").required(true).index(1)))
		.subcommand(Command::new("disactivate").arg(Arg::new("target").required(true).index(1)))
		.subcommand(Command::new("status").arg(Arg::new("target").required(true).index(1)))
		.subcommand(Command::new("config").arg(Arg::new("command").required(true).index(1)));

	(cmd.clone(), cmd.clone().get_matches())
}
