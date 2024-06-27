#[rustfmt::skip]

macro_rules! throw {
	($message:expr, exit) => {{
		color_print::cprintln!("<red,bold>Error:</> {}", $message);
		std::process::exit(1);
	}};
	($message:expr) => {{
		color_print::cprintln!("<red,bold>Error:</> {}", $message);
	}};
}

pub(crate) use throw;

pub const SKINNER_FILE_NOT_FOUND: &str = "Skinner file not found";
pub const ERROR_NOT_ROOT: &str = "You must be root to run this command";
