use std::process::Command;

use anyhow::{Context, Result};

use crate::cli::Args;
use crate::inputs::Inputs;
use crate::utils::output_success;

pub fn commit_changes(args: &Args, inputs: &Inputs) -> Result<()> {
	println!();
	let git_program = "git";
	if args.all {
		Command::new(git_program)
			.args(&["add", "."])
			.output()
			.context("Failed to stages all changes")?;
		output_success("Staged all changes");
	}
	Command::new(git_program)
		.args(&["commit", "-m", &message(inputs)])
		.output()
		.context("Failed to commit changes")?;
	output_success("Committed changes");
	if args.push {
		Command::new(git_program)
			.args(&["push"])
			.output()
			.context("Failed to push changes")?;
		output_success("Pushed changes");
	}
	Ok(())
}

fn message(inputs: &Inputs) -> String {
	let fmt_scope = format!("({})", inputs.scope);
	format!(
		"{}{}{}: {}

{}

{}",
		inputs.change_type,
		if inputs.scope == "none" {
			""
		} else {
			&fmt_scope
		},
		if inputs.breaking_changes == "" {
			""
		} else {
			"!"
		},
		inputs.description,
		inputs.long_description,
		inputs.breaking_changes
	)
	.trim()
	.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_message() {
		// Simple message
		assert_eq!(
			message(&Inputs {
				change_type: String::from("feat"),
				scope: String::from("none"),
				description: String::from("add polish language"),
				long_description: String::from(""),
				breaking_changes: String::from("")
			}),
			String::from("feat: add polish language")
		);
		// With Scope
		assert_eq!(
			message(&Inputs {
				change_type: String::from("feat"),
				scope: String::from("lang"),
				description: String::from("add polish language"),
				long_description: String::from(""),
				breaking_changes: String::from("")
			}),
			String::from("feat(lang): add polish language")
		);
		// Long description
		assert_eq!(
			message(&Inputs {
				change_type: String::from("feat"),
				scope: String::from("lang"),
				description: String::from("add polish language"),
				long_description: String::from(
					"added this language because it is super cool and we need more languages like \
					 it."
				),
				breaking_changes: String::from("")
			}),
			String::from(
				"feat(lang): add polish language\n\nadded this language because it is super cool \
				 and we need more languages like it."
			)
		);
		// Breaking changes
		assert_eq!(
			message(&Inputs {
				change_type: String::from("feat"),
				scope: String::from("lang"),
				description: String::from("add polish language"),
				long_description: String::from(
					"added this language because it is super cool and we need more languages like \
					 it."
				),
				breaking_changes: String::from("This breaks some other languages.")
			}),
			String::from(
				"feat(lang)!: add polish language\n\nadded this language because it is super cool \
				 and we need more languages like it.\n\nThis breaks some other languages."
			)
		);
	}
}
