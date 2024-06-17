pub mod l;
pub mod r;
pub mod x;

#[derive(Debug)]
pub struct Metadata<'a> {
	pub target: &'a str,
	pub replaced_with: String,
	pub skinner: &'a str,
	pub status: bool,
	pub last_use_date: Option<String>,
	pub last_use_cmd: Option<String>,
	pub created_at: Option<String>,
}
