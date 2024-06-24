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

// Common errors

pub const ERROR_NOT_ROOT: &str = "This operation requires root privileges";

pub const TARGET_UNDEACTIVATABLE: &str =
	"The target does not exist or may have been deactivated previously";
pub const TARGET_UNACTIVATABLE: &str =
	"The target does not exist in the database, or it is activated";
pub const CANNOT_DISACTIVATE_BUILTIN: &str = "Cannot deactivate a built-in command";
pub const NOT_EXISTING_ENTITY: &str = "The target was not recognized as an existing entity";
pub const TARGET_METADATA_EMPTY: &str = "Target metadata is empty";
pub const REPLACEMENT_FILE_NOT_FOUND: &str = "The replacement file for the target does not exist";

pub const FILE_NOT_FOUND: &str = "File does not exist";

pub const SKINNER_NOT_EXIST: &str = "Skinner does not exist";
