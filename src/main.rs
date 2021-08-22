use alarm_clock::{Config, run};

fn main() {
	let config = Config::new(&std::env::args().collect::<Vec<String>>()).unwrap_or_else(|err| {
		eprintln!("Could not parse arguments: {:?}", err);
		std::process::exit(1);
	});

	println!("{:?}", config);
	run();
}
