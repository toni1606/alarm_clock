use getopt::Opt;

#[derive(Debug, Eq, PartialEq)]
enum Period {
	Am,
	Pm
}

#[derive(Debug)]
pub struct Config {
	hour: u32,
	minute: u32,
	time_zone: Period
}

pub struct Time {
	hour: u32,
	minute: u32
}

#[derive(Debug)]
pub struct TimeError {
	details: String
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {
		let mut opts = getopt::Parser::new(args, "h:m:p");

		let mut hour_flag: Option<String> = None;
		let mut minute_flag: Option<String> = None;
		let mut is_pm: Period = Period::Am;
		
		loop {
			match opts.next().transpose()? {
				None => break,
				Some(opt) => {
					match opt {
						Opt('h', Some(hour)) => hour_flag = Some(hour),
						Opt('m', Some(minute)) => minute_flag = Some(minute),
						Opt('p', None) => is_pm = Period::Pm,
						_ => unreachable!()
					}
				}
			}
		}

		let hour_flag: u32 = match hour_flag {
			None => {
				eprintln!("No hour value given, reverting to default value (0)");
				0
			},
			Some(h) => {
				let tem = h.parse()?;
				if tem >= 24 {
					return Err(Box::new(TimeError::new("Hour was greater or equal to 24")));
				}
				tem
			}
		};

		let minute_flag: u32 = match minute_flag {
			None => {
				eprintln!("No minute value given, reverting to default value (0)");
				0
			},
			Some(m) => {
				let tem = m.parse()?;
				if tem >= 60 {
					return Err(Box::new(TimeError::new("Minute was greater or equal to 60")));
				}
				tem
			}
		};

		Ok(Config {
			hour: hour_flag, 
			minute: minute_flag,
			time_zone: is_pm
		})
	}
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