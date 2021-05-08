use std::env;

mod cli;
mod conf;
mod git;
mod inputs;
mod utils;

fn main() {
	let args = cli::parse_args(env::args().collect());
	let config = conf::read().expect("Failed to read from configuration file");
	let inputs = inputs::get_inputs(&config).expect("Failed to get scope");
	git::commit_changes(&args, &inputs).expect("Failed to commit changes");
}
