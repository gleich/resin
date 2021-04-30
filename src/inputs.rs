use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

use crate::conf;

pub struct Inputs {
	pub change_type: String,
	pub scope: String,
	pub description: String,
	pub long_description: String,
	pub breaking_changes: String,
}

pub fn get_inputs(config: &conf::Config) -> Result<Inputs> {
	let theme = ColorfulTheme::default();
	let change_types = &[
		"feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
		"revert",
	];

	let change_type_selection = Select::with_theme(&theme)
		.with_prompt("What is the change type?")
		.default(0)
		.items(change_types)
		.interact()
		.context("Failed to present change type selection to user")?;
	let scope_selection = Select::with_theme(&theme)
		.with_prompt("What is the scope?")
		.default(0)
		.items(&config.scopes)
		.interact()
		.context("Failed to present scope selection to user")?;
	let description: String = Input::with_theme(&theme)
		.with_prompt("What is the description?")
		.interact()
		.context("Failed to ask for description")?;
	let long_description: String = Input::with_theme(&theme)
		.allow_empty(true)
		.with_prompt("What is the longer description?")
		.interact()
		.context("Failed to ask for longer description")?;
	let breaking_changes: String = Input::with_theme(&theme)
		.allow_empty(true)
		.with_prompt("What are some breaking change (if any)?")
		.interact()
		.context("Failed to ask for breaking changes")?;
	Ok(Inputs {
		change_type: String::from(change_types[change_type_selection]),
		scope: config.scopes.get(scope_selection).unwrap().to_owned(),
		description,
		long_description,
		breaking_changes,
	})
}
