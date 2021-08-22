use alarm_clock::TimeZone;
use getopt::Opt;

fn main() {
	parser();
}

fn parser() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = std::env::args().collect();
	let mut opts = getopt::Parser::new(&args, "h:m:p:");

	let mut hour_flag: Option<String> = None;
	let mut minute_flag: Option<String> = None;
	let mut is_pm: Option<TimeZone> = None;
	
	loop {
		match opts.next().transpose()? {
			None => break,
			Some(opt) => {
				match opt {
					Opt('h', Some(hour)) => hour_flag = Some(hour),
					Opt('m', Some(minute)) => minute_flag = Some(minute),
					Opt('p', Some(pm)) => {
						if &pm == "true" {
							is_pm = Some(TimeZone::Pm);
						} else {
							is_pm = Some(TimeZone::Am);
						}
					},
					_ => unreachable!()
				}
			}
		}
	}

	let hour_flag: u8 = match hour_flag {
		None => 0,
		Some(h) => h.parse()?
	};

	let minute_flag: u8 = match minute_flag {
		None => 0,
		Some(m) => m.parse()?
	};

	let is_pm: TimeZone = match is_pm {
		Some(TimeZone::Am) => TimeZone::Am,
		_ => TimeZone::Pm
	};

	let config = (hour_flag, minute_flag, is_pm);

	println!("{:?}", config);

	Ok(())
}
