use std::path::PathBuf;
use std::process::{Command, ExitStatus, exit};
use std::{env, fs};

use anyhow::{Context, Result, anyhow, bail};
use clap::ArgMatches;
use configparser::ini::Ini;
use dialoguer::Confirm;
use directories::UserDirs;

use crate::conf::Config;
use crate::inputs::{self, Inputs};
use crate::utils::{output_failure, output_success};

const GIT: &str = "git";

pub fn commit_changes(conf: &Config, args: &ArgMatches, inputs: &Inputs) -> Result<()> {
	println!();
	if args.get_flag("all") {
		let status = Command::new(GIT)
			.args(["add", "--verbose", "."])
			.status()
			.context("Failed to stage all changes")?;
		check_status(status, "stage all changes");
		output_success("Staged all changes\n");
	}

	let status = Command::new(GIT)
		.args(["commit", "-m", &message(conf, inputs)?])
		.status()
		.context("Failed to commit changes")?;
	check_status(status, "commit changes");
	output_success("Committed changes");

	if args.get_flag("push") {
		println!();
		let status = Command::new(GIT)
			.args(["push"])
			.status()
			.context("Failed to push changes")?;
		check_status(status, "push changes");
		output_success("Pushed changes");
	}
	Ok(())
}

pub fn check_location() -> Result<()> {
	let output = Command::new(GIT)
		.args(["rev-parse", "--show-toplevel"])
		.output()
		.context("Failed to get repoisotry toplevel directory")?;
	check_status(output.status, "finding repo top level directory");
	let cwd = env::current_dir().context("Failed to get current working directory")?;
	let repo_top_dir = String::from_utf8_lossy(&output.stdout)
		.trim_end()
		.to_string();
	if repo_top_dir != cwd.to_str().unwrap() {
		let go_to_root = Confirm::with_theme(&*inputs::THEME)
			.default(false)
			.with_prompt("Do you want to run this command from the root of this repository?")
			.interact()
			.context("Failed to ask if user wants to run from root of repository")?;
		if go_to_root {
			env::set_current_dir(PathBuf::from(repo_top_dir))
				.context("failed to go to root directory")?;
		}
	}
	Ok(())
}

fn check_status(status: ExitStatus, task: &str) {
	if !status.success() {
		output_failure(
			format!(
				"Failed to {}. Try running the command manually and resolving the error",
				task
			)
			.as_str(),
		);
		exit(1);
	}
}

fn message(conf: &Config, inputs: &Inputs) -> Result<String> {
	let fmt_scope = if conf.parentheses {
		format!("({})", inputs.scope)
	} else {
		format!("[{}]", inputs.scope)
	};
	let message = format!(
		"{}{}{}: {}

{}

{}

{}",
		inputs.change_type,
		if inputs.scope == "none" {
			""
		} else {
			&fmt_scope
		},
		if inputs.breaking_changes.is_empty() {
			""
		} else {
			"!"
		},
		inputs.description,
		inputs.long_description,
		inputs.breaking_changes,
		if conf.sign { signoff()? } else { String::new() },
	)
	.trim()
	.to_string();
	Ok(message)
}

/// Create the sign off message based off the user's ~/.gitconfig
fn signoff() -> Result<String> {
	let gitconfig_path = UserDirs::new().unwrap().home_dir().join(".gitconfig");

	// Reading the values
	if gitconfig_path.exists() {
		let mut config = Ini::new();
		config
			.read(fs::read_to_string(gitconfig_path)?)
			.map_err(|e| anyhow!("Failed to read from gitconfig file: {}", e))?;

		let user_group = "user";
		let name = config.get(user_group, "name").unwrap_or_default();
		if name.is_empty() {
			return Err(anyhow!(
				"No value provided for name in the user section of your gitconfig"
			));
		}
		let email = config.get(user_group, "email").unwrap_or_default();
		if email.is_empty() {
			return Err(anyhow!(
				"No value provided for email in the user section of your gitconfig"
			));
		}
		return Ok(format!("Signed-off-by: {} <{}>", name, email));
	}
	bail!("Failed to find ~/.config for signoff message. Does it exist?")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_message() -> Result<()> {
		// simple message
		assert_eq!(
			message(
				&Config {
					scopes: vec![],
					sign: false,
					parentheses: false,
				},
				&Inputs {
					change_type: String::from("feat"),
					scope: String::from("none"),
					description: String::from("add polish language"),
					long_description: String::from(""),
					breaking_changes: String::from(""),
				}
			)?,
			String::from("feat: add polish language")
		);
		// With Scope
		assert_eq!(
			message(
				&Config {
					scopes: vec![],
					sign: false,
					parentheses: false,
				},
				&Inputs {
					change_type: String::from("feat"),
					scope: String::from("lang"),
					description: String::from("add polish language"),
					long_description: String::from(""),
					breaking_changes: String::from("")
				}
			)?,
			String::from("feat[lang]: add polish language")
		);
		// with scope and parentheses
		assert_eq!(
			message(
				&Config {
					scopes: vec![],
					sign: false,
					parentheses: true,
				},
				&Inputs {
					change_type: String::from("feat"),
					scope: String::from("lang"),
					description: String::from("add polish language"),
					long_description: String::from(""),
					breaking_changes: String::from("")
				}
			)?,
			String::from("feat(lang): add polish language")
		);
		// long description
		assert_eq!(
			message(
				&Config {
					scopes: vec![],
					sign: false,
					parentheses: false,
				},
				&Inputs {
					change_type: String::from("feat"),
					scope: String::from("lang"),
					description: String::from("add polish language"),
					long_description: String::from(
						"added this language because it is super cool and we need more languages \
						 like it."
					),
					breaking_changes: String::from("")
				}
			)?,
			String::from(
				"feat[lang]: add polish language\n\nadded this language because it is super cool \
				 and we need more languages like it."
			)
		);
		// breaking changes
		assert_eq!(
			message(
				&Config {
					scopes: vec![],
					sign: false,
					parentheses: false,
				},
				&Inputs {
					change_type: String::from("feat"),
					scope: String::from("lang"),
					description: String::from("add polish language"),
					long_description: String::from(
						"added this language because it is super cool and we need more languages \
						 like it."
					),
					breaking_changes: String::from("This breaks some other languages.")
				}
			)?,
			String::from(
				"feat[lang]!: add polish language\n\nadded this language because it is super cool \
				 and we need more languages like it.\n\nThis breaks some other languages."
			)
		);
		Ok(())
	}
}
