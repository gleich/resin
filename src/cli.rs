use std::process;

#[derive(Debug, PartialEq)]
pub struct Args {
	pub all: bool,
	pub push: bool,
}

pub fn parse_args(args: Vec<String>) -> Args {
	if has_flag(&args, "help", 'h') {
		println!(
			"resin :: cli interface for conventional commits

flags:
    --help    (-h) showing this help page
    --all     (-a) If all files should be added
    --push    (-p) If the changes should be pushed after running"
		);
		process::exit(0);
	}

	Args {
		push: has_flag(&args, "push", 'p'),
		all: has_flag(&args, "all", 'a'),
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
		assert_eq!(
			parse_args(to_string_vec(vec![""])),
			Args {
				all: false,
				push: false
			}
		);
		// shorthand arg
		assert_eq!(
			parse_args(to_string_vec(vec!["-p"])),
			Args {
				push: true,
				all: false
			}
		);
		// full arg
		assert_eq!(
			parse_args(to_string_vec(vec!["--push"])),
			Args {
				push: true,
				all: false
			}
		);

		// args but no flags
		assert_eq!(
			parse_args(to_string_vec(vec!["foo", "bar"])),
			Args {
				push: false,
				all: false
			}
		);
		// multiple
		assert_eq!(
			parse_args(to_string_vec(vec!["-p", "--all"])),
			Args {
				push: true,
				all: true
			}
		);
	}

	#[test]
	fn test_has_flag() {
		let name = "help";
		let shorthand = 'h';
		assert_eq!(has_flag(&vec![], name, shorthand), false);
		assert_eq!(
			has_flag(&to_string_vec(vec!["foo", "bar"]), name, shorthand),
			false
		);
		assert_eq!(has_flag(&vec![String::from("-h")], name, shorthand), true);
		assert_eq!(
			has_flag(&vec![String::from("--help")], name, shorthand),
			true
		);
	}
}
