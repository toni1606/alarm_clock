use getopt::Opt;
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug, Eq, PartialEq)]
enum TimeZone {
	Am,
	Pm
}

#[derive(Debug)]
pub struct Config {
	hour: u8,
	minute: u8,
	time_zone: TimeZone
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {
		let mut opts = getopt::Parser::new(args, "h:m:p");

		let mut hour_flag: Option<String> = None;
		let mut minute_flag: Option<String> = None;
		let mut is_pm: TimeZone = TimeZone::Am;
		
		loop {
			match opts.next().transpose()? {
				None => break,
				Some(opt) => {
					match opt {
						Opt('h', Some(hour)) => hour_flag = Some(hour),
						Opt('m', Some(minute)) => minute_flag = Some(minute),
						Opt('p', None) => is_pm = TimeZone::Pm,
						_ => unreachable!()
					}
				}
			}
		}

		let hour_flag: u8 = match hour_flag {
			None => {
				eprintln!("No hour value given, reverting to default value (0)");
				0
			},
			Some(h) => h.parse()?
		};

		let minute_flag: u8 = match minute_flag {
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

pub fn run(config: Config, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
	let random_url_index = rand::thread_rng().gen_range(1..(get_lines(filename)? + 1));	
	
	println!("{}", random_url_index);
	Ok(())
}

fn get_lines(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
	let reader = BufReader::new(File::open(filename)?);
	let mut count = 0;
	
	for _ in reader.lines() {
		count += 1;
	}

	Ok(count)
}