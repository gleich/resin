use std::fs;
use std::path::Path;

use anyhow::Result;
use directories::UserDirs;
use itertools::Itertools;
use serde::Deserialize;

use crate::utils::to_string_vec;

#[derive(Debug, Deserialize)]
struct RawTOML {
	scopes: Option<Vec<String>>,
	sign: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub struct Config {
	pub scopes: Vec<String>,
	pub sign: bool,
}

pub fn read() -> Result<Config> {
	let fnames = ["resin", "commits", "conventional_commits"];
	let mut config = Config {
		scopes: to_string_vec(vec![
			"none", "lint", "deps", "release", "remove", "licence", "config", "scripts",
		]),
		sign: false,
	};

	// Reading global config file for user
	let raw_global_path = &format!(
		"{}/.config/resin/config.toml",
		UserDirs::new().unwrap().home_dir().display()
	);
	let global_path = Path::new(&raw_global_path);
	if global_path.exists() {
		let content = fs::read_to_string(global_path)?;
		let raw_data: RawTOML = toml::from_str(&content)?;
		config.sign = raw_data.sign.unwrap_or_default();
		config.scopes.extend(raw_data.scopes.unwrap_or_default());
	};

	// Reading local config file
	for i in 0..fnames.len() {
		let file_name = format!("{}{}", fnames[i], ".toml");
		let path = Path::new(&file_name);

		if path.exists() {
			let content = fs::read_to_string(path)?;
			let raw_data: RawTOML = toml::from_str(&content)?;
			config.scopes.extend(raw_data.scopes.unwrap_or_default());
			if !raw_data.sign.is_none() {
				config.sign = raw_data.sign.unwrap();
			}
			break;
		}
	}

	// Removing duplicates
	config.scopes = config.scopes.into_iter().unique().collect();

	Ok(config)
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;

	use crate::utils::to_string_vec;

	#[test]
	fn test_read() -> Result<()> {
		assert_eq!(
			read()?,
			Config {
				scopes: to_string_vec(vec![
					"none",
					"lint",
					"deps",
					"release",
					"remove",
					"license",
					"config",
					"scripts",
					"docker",
					"github actions"
				]),
				sign: false,
			},
		);
		Ok(())
	}
}
