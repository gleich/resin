use clap::{Arg, Command};

pub fn setup() -> Command {
	Command::new("resin")
		.version("1.6.7")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("Fast CLI for conventional commits\nhttps://github.com/gleich/resin")
		.arg(
			Arg::new("all")
				.help("Stage all changes in the current directory before committing")
				.short('a')
				.long("all")
				.num_args(0),
		)
		.arg(
			Arg::new("push")
				.help("Push changes after committing")
				.short('p')
				.long("push")
				.num_args(0),
		)
}
