use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{FuzzySelect, Input};

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

	let change_type_selection = FuzzySelect::with_theme(&theme)
		.with_prompt("Type")
		.default(0)
		.items(change_types)
		.interact()
		.context("Failed to present change type selection to user")?;
	let scope_selection = FuzzySelect::with_theme(&theme)
		.with_prompt("Scope")
		.default(0)
		.items(&config.scopes)
		.interact()
		.context("Failed to present scope selection to user")?;
	let description: String = Input::with_theme(&theme)
		.with_prompt("Description")
		.interact_text()
		.context("Failed to ask for description")?;
	let long_description: String = Input::with_theme(&theme)
		.allow_empty(true)
		.with_prompt("Longer description (optional)")
		.interact_text()
		.context("Failed to ask for longer description")?;
	let breaking_changes: String = Input::with_theme(&theme)
		.allow_empty(true)
		.with_prompt("Breaking change (optional)")
		.interact_text()
		.context("Failed to ask for breaking changes")?;
	Ok(Inputs {
		change_type: String::from(change_types[change_type_selection]),
		scope: config.scopes.get(scope_selection).unwrap().to_owned(),
		description,
		long_description,
		breaking_changes,
	})
}
