use std::process;

use anyhow::Result;

pub struct Args {
	pub verbose: bool,
}

pub fn parse_args(args: Vec<String>) -> Result<Args> {
	if has_flag(&args, "help", "h") {
		println!(
			"stabit :: cli interface for conventional commits

flags:
    --help    (-h) showing this help page
    --verbose (-v) show full output (default: off)"
		);
		process::exit(0);
	}

	Ok(Args {
		verbose: has_flag(&args, "verbose", "v"),
	})
}

fn has_flag(args: &Vec<String>, name: &str, shorthand: &str) -> bool {
	args.contains(&format!("--{}", name)) || args.contains(&format!("-{}", shorthand))
}
