use crate::helper::{activate, disactivate, run, status};

pub enum Action {
	Activate,
	Disactivate,
	Status,
}

pub fn db_command() {}

#[inline]
pub fn run_command(target: &str, extras: &str) {
	let _ = run::action(target, extras);
}

pub fn bar_command(action: &str, target: &str) {
	let action_enum = match action {
		"activate" => Action::Activate,
		"disactivate" => Action::Disactivate,
		"status" => Action::Status,
		_ => {
			println!("Invalid action!");
			return;
		}
	};

	let _ = match action_enum {
		Action::Activate => activate::action(target),
		Action::Disactivate => disactivate::action(target),
		Action::Status => status::action(target),
	};
}
