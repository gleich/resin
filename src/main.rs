use std::env;

mod cli;
mod conf;
mod utils;

fn main() {
	cli::parse_args(env::args().collect());
	let config = conf::read().expect("Failed to read from configuration file");
	println!("scopes:   {:?}", config.scopes);
	println!("brackets: {:?}", config.brackets)
}
