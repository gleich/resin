use clap::{App, Arg};

#[derive(Debug, PartialEq)]
pub struct Args {
	pub all: bool,
	pub push: bool,
}

pub fn parse_args() -> Args {
	let matches = App::new("resin")
		.version("1.2.3")
		.author("Matthew Gleich <email@mattglei.ch>")
		.about("Superfast CLI interface for the conventional commits commit format")
		.arg(
			Arg::with_name("all")
				.help("Run git add . before committing the the changes")
				.short("a")
				.long("all"),
		)
		.arg(
			Arg::with_name("push")
				.help("Run git push after committing the changes")
				.short("p")
				.long("push"),
		)
		.get_matches();

	Args {
		all: matches.occurrences_of("all") > 0,
		push: matches.occurrences_of("push") > 0,
	}
}
