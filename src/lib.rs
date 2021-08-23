use getopt::Opt;
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use chrono::prelude::*;

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

struct Time {
	hour: u32,
	minute: u32
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
			Some(h) => h.parse()?
		};

		let minute_flag: u32 = match minute_flag {
			None => {
				eprintln!("No minute value given, reverting to default value (0)");
				0
			},
			Some(m) => m.parse()?
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
}

pub fn run(config: Config, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
	let time = Time::from_config(&config);

	let random_url_index = rand::thread_rng().gen_range(1..(get_line_count(filename)? + 1));	
	
	let local_time = Local::now();
	let execution_time = Local.ymd(
							local_time.year(),
							local_time.month(),
							local_time.day())
							.and_hms(time.hour, time.minute, 0);

	std::thread::sleep((execution_time - local_time).to_std()?);
	
	Ok(())
}

fn get_line_count(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
	let reader = BufReader::new(File::open(filename)?);
	let mut count = 0;
	
	for _ in reader.lines() {
		count += 1;
	}

	Ok(count)
}