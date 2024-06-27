pub mod external;
pub mod l;
pub mod r;
pub mod x;

use skin_builder::makefile;

use crate::error;

enum Skinner {
	L,
	R,
	X,
	External,
}

#[derive(Debug, Clone)]
pub struct Metadata<'a> {
	pub target: &'a str,
	pub replaced_with: Option<String>,
	pub skinner: &'a str,
	pub status: bool,
	pub last_use_date: Option<String>,
	pub last_use_cmd: Option<String>,
	pub created_at: Option<String>,
}

pub fn is_skinner_exist(skinner: &str) -> bool {
	match skinner.to_uppercase().as_str() {
		"L" | "R" | "X" => true,
		&_ => false,
	}
}

pub fn run_by_skinner(
	target: &str,
	skinner: &str,
	extras: Option<&str>,
) -> Result<bool, Box<dyn std::error::Error>> {
	let extras_value: &str = extras.unwrap_or_default();

	let skinners_enum = match skinner.to_uppercase().as_str() {
		"L" => Skinner::L,
		"R" => Skinner::R,
		"X" => Skinner::X,
		_ => {
			if !makefile::is_skinner_exist(skinner) {
				error::throw!(error::SKINNER_NOT_EXIST);
				return Ok(false);
			}
			Skinner::External
		}
	};

	match skinners_enum {
		Skinner::L => l::action_run(target, extras_value),
		Skinner::R => r::action_run(target, extras_value),
		Skinner::X => x::action_run(target, extras_value),
		Skinner::External => external::action_run(target, extras_value, skinner),
	}
}

pub fn r#do<'a>(target: &'a str, skinner: &'a str) -> Option<Metadata<'a>> {
	let skinners_enum = match skinner.to_uppercase().as_str() {
		"L" => Skinner::L,
		"R" => Skinner::R,
		"X" => Skinner::X,
		_ => {
			if !makefile::is_skinner_exist(skinner) {
				error::throw!(error::SKINNER_NOT_EXIST);
				return None;
			}
			Skinner::External
		}
	};

	match skinners_enum {
		Skinner::L => l::r#do(target),
		Skinner::R => r::r#do(target),
		Skinner::X => x::r#do(target),
		Skinner::External => external::r#do(target, skinner),
	}
}

pub fn undo(metadata: Metadata) -> bool {
	let skinners_enum = match metadata.skinner.to_uppercase().as_str() {
		"L" => Skinner::L,
		"R" => Skinner::R,
		"X" => Skinner::X,
		_ => {
			if !makefile::is_skinner_exist(metadata.skinner) {
				error::throw!(error::SKINNER_NOT_EXIST);
				return false;
			}
			Skinner::External
		}
	};

	match skinners_enum {
		Skinner::L => l::undo(metadata),
		Skinner::R => r::undo(metadata),
		Skinner::X => x::undo(metadata),
		Skinner::External => external::undo(metadata),
	}
}
