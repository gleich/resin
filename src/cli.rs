use std::process;

#[derive(Debug, PartialEq)]
pub struct Args {
	pub verbose: bool,
}

pub fn parse_args(args: Vec<String>) -> Args {
	if has_flag(&args, "help", "h") {
		println!(
			"stabit :: cli interface for conventional commits

flags:
    --help    (-h) showing this help page
    --verbose (-v) show full output (default: off)"
		);
		process::exit(0);
	}

	Args {
		verbose: has_flag(&args, "verbose", "v"),
	}
}

fn has_flag(args: &Vec<String>, name: &str, shorthand: &str) -> bool {
	args.contains(&format!("--{}", name)) || args.contains(&format!("-{}", shorthand))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::to_string_vec;

	#[test]
	fn test_parse_args() {
		assert_eq!(parse_args(to_string_vec(vec![""])), Args { verbose: false });
		assert_eq!(
			parse_args(to_string_vec(vec!["-v"])),
			Args { verbose: true }
		);
		assert_eq!(
			parse_args(to_string_vec(vec!["--verbose"])),
			Args { verbose: true }
		);
		assert_eq!(
			parse_args(to_string_vec(vec!["random", "content"])),
			Args { verbose: false }
		);
	}
}
