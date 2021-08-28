use crate::datatypes::config::Config;
use crate::datatypes::period::Period;

pub struct Time {
	hour: u32,
	minute: u32
}

impl Time {
	pub fn new(hour: u32, minute: u32) -> Time {
		Time {hour, minute}
	}

	pub fn from_config(config: &Config) -> Time {
		let mut _24_day_format: u32 = config.get_hour();
		if config.get_time_zone() == Period::Pm {
			_24_day_format = config.get_hour() + 12;
		}

		Time {
			hour: _24_day_format,
			minute: config.get_minute()
		}
	}

	pub fn get_hour(&self) -> u32 {
		self.hour
	}

	pub fn get_minute(&self) -> u32 {
		self.minute
	}
}