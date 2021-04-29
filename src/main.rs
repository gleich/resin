use std::env;

mod cli;
mod conf;

fn main() {
	cli::parse_args(env::args().collect()).expect("Failed to parse args");
	let config = conf::read().expect("Failed to read from configuration file");
	println!("scopes:   {:?}", config.scopes);
	println!("brackets: {:?}", config.brackets)
}
