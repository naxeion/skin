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

pub const ERROR_NOT_ROOT: &str = "This operation requires root privileges";
pub const NOT_EXISTING_ENTITY: &str = "The target was not recognized as an existing entity";
pub const TARGET_METADATA_EMPTY: &str = "Target metadata is empty";
