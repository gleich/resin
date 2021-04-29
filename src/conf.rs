use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawTOML {
	scopes: Option<Vec<String>>,
	brackets: Option<bool>,
}
pub struct Config {
	pub scopes: Vec<String>,
	pub brackets: bool,
}

pub fn read() -> Result<Config, anyhow::Error> {
	let fnames = ["stabit", "commits", "conventional_commits"];
	let mut scopes = vec![
		String::from("lint"),
		String::from("deps"),
		String::from("release"),
		String::from("remove"),
		String::from("license"),
		String::from("config"),
		String::from("scripts"),
	];

	for i in 0..fnames.len() {
		let file_name = format!("{}{}", fnames[i], ".toml");
		let path = Path::new(&file_name);

		if path.exists() {
			let content = fs::read_to_string(path)?;
			let raw_data: RawTOML = toml::from_str(&content)?;
			scopes.extend(raw_data.scopes.unwrap_or_default());
			return Ok(Config {
				scopes,
				brackets: raw_data.brackets.unwrap_or_default(),
			});
		}
	}
	Ok(Config {
		scopes,
		brackets: true,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_read() -> Result<(), anyhow::Error> {
		let result = read()?;
		assert_eq!(
			result.scopes,
			vec![
				"lint", "deps", "release", "remove", "license", "config", "scripts", "docker",
				"github", "actions"
			]
		);
		assert_eq!(result.brackets, false);
		Ok(())
	}
}
