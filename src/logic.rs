use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use chrono::prelude::{Local, Datelike, TimeZone};
use webbrowser::open;

use crate::datatypes::Config;
use crate::datatypes::Time;

pub fn run(config: Config, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
	let time = Time::from_config(&config);

	let random_url_index = rand::thread_rng().gen_range(1..(get_line_count(filename)? + 1));	
	
	let local_time = Local::now();
	let execution_time = Local.ymd(
								local_time.year(),
								local_time.month(),
								local_time.day())
							.and_hms(time.get_hour(), time.get_minute(), 0);

	std::thread::sleep((execution_time - local_time).to_std()?);
	

	open(&get_line_contents(filename, random_url_index)?)?;
	Ok(())
}

fn get_line_count(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
	let reader = BufReader::new(File::open(filename)?);
	let mut count = 0;
	
	for _ in reader.lines() {
		count += 1;
	}

	Ok(count)
}

fn get_line_contents(filename: &str, index: usize) -> Result<String, Box<dyn std::error::Error>> {
	let reader = BufReader::new(File::open(filename)?);

	for (i, line) in reader.lines().enumerate() {
		if i == index {
			return Ok(line?)
		}
	}

	unreachable!();
}