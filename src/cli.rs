use clap::{Arg, Command};

pub fn setup() -> Command {
	Command::new("resin")
		.version("1.6.2")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("Fast CLI for conventional commits")
		.arg(
			Arg::new("all")
				.help("Run git add . before committing the the changes")
				.short('a')
				.long("all")
				.num_args(0),
		)
		.arg(
			Arg::new("push")
				.help("Run git push after committing the changes")
				.short('p')
				.long("push")
				.num_args(0),
		)
}
