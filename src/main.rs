use getopt::Opt;

fn main() {
	rd();
}

fn rd() -> Result<(), Box<dyn std::error::Error>>{
	let mut args: Vec<String> = std::env::args().collect();
	let mut opts = getopt::Parser::new(&args, "h:m:p:");

	loop {
		match opts.next().transpose()? {
			None => break,
			Some(opt) => {
				match opt {
					Opt('h', Some(hour)) => println!("h: {}", hour),
					Opt('m', Some(minute)) => println!("m: {}", minute),
					Opt('p', Some(pm)) => {
						if &pm == "true" {
							println!("p: true");
						} else if &pm == "false" {
							println!("p: false");
						} else {
							println!("p: g");
						}
					},
					Opt('p', None) => println!("p: None"),
					_ => unreachable!()
				}
			}
		}
	}

	Ok(())
}
