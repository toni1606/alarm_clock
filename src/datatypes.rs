
#[derive(Debug, Eq, PartialEq)]
enum Period {
	Am,
	Pm
}



pub struct Time {
	hour: u32,
	minute: u32
}

#[derive(Debug)]
pub struct TimeError {
	details: String
}



impl Time {
	pub fn new(hour: u32, minute: u32) -> Time {
		Time {hour, minute}
	}

	pub fn from_config(config: &Config) -> Time {
		let mut _24_day_format: u32 = config.hour;
		if config.time_zone == Period::Pm {
			_24_day_format = config.hour + 12;
		}

		Time {
			hour: _24_day_format,
			minute: config.minute
		}
	}

	pub fn get_hour(&self) -> u32 {
		self.hour
	}

	pub fn get_minute(&self) -> u32 {
		self.minute
	}
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