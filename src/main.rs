use std::env;

mod cli;
mod conf;
mod inputs;
mod utils;

fn main() {
	cli::parse_args(env::args().collect());
	let config = conf::read().expect("Failed to read from configuration file");
	inputs::get_inputs(&config).expect("Failed to get scope");
}
