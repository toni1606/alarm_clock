#[derive(Debug)]
pub struct TimeError {
	details: String
}

impl TimeError {
	pub fn new(s: &str) -> TimeError {
		TimeError {details: String::from(s)}
	}
}

impl std::fmt::Display for TimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.details)
	}
}

impl std::error::Error for TimeError {
	fn description(&self) -> &str {
		&self.details
	}
}