use alarm_clock::{Config, run};

fn main() {
	let config = Config::new(&std::env::args().collect::<Vec<String>>()).unwrap_or_else(|err| {
		eprintln!("Could not parse arguments: {:?}", err);
		std::process::exit(1);
	});

	run(config, "data/url.txt").unwrap_or_else(|err| {
		eprintln!("An error ocurred during the execution of the program: {}", err);
		std::process::exit(2);
	});
}
