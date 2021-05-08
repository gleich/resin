use std::process;

#[derive(Debug, PartialEq)]
pub struct Args {
	pub verbose: bool,
}

pub fn parse_args(args: Vec<String>) -> Args {
	if has_flag(&args, "help", 'h') {
		println!(
			"resin :: cli interface for conventional commits

flags:
    --help    (-h) showing this help page
    --verbose (-v) show full output (default: off)"
		);
		process::exit(0);
	}

	Args {
		verbose: has_flag(&args, "verbose", 'v'),
	}
}

fn has_flag(args: &Vec<String>, name: &str, shorthand: char) -> bool {
	args.contains(&format!("--{}", name)) || args.contains(&format!("-{}", shorthand))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::to_string_vec;

	#[test]
	fn test_parse_args() {
		// no args
		assert_eq!(parse_args(to_string_vec(vec![""])), Args { verbose: false });
		// shorthand arg
		assert_eq!(
			parse_args(to_string_vec(vec!["-v"])),
			Args { verbose: true }
		);
		// full name arg
		assert_eq!(
			parse_args(to_string_vec(vec!["--verbose"])),
			Args { verbose: true }
		);
		// args but no flags
		assert_eq!(
			parse_args(to_string_vec(vec!["foo", "bar"])),
			Args { verbose: false }
		);
	}

	#[test]
	fn test_has_flag() {
		let name = "verbose";
		let shorthand = 'v';
		assert_eq!(has_flag(&vec![], name, shorthand), false);
		assert_eq!(
			has_flag(&to_string_vec(vec!["foo", "bar"]), name, shorthand),
			false
		);
		assert_eq!(has_flag(&vec![String::from("-v")], name, shorthand), true);
		assert_eq!(
			has_flag(&vec![String::from("--verbose")], name, shorthand),
			true
		);
	}
}
