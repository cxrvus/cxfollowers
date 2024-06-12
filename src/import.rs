use std::{fs, path::PathBuf};
use anyhow::{anyhow, Result};
use regex::Regex;

const ZIP_PATTERN: &'static str = r#"instagram-.+?-\d{4}-\d{2}-\d{2}-\w{8}\.zip"#;
const EXAMPLE_FILE_NAME: &'static str = "instagram-my_username-2021-01-01-aBcD1X2Y.zip";

pub fn import_zip(path: PathBuf) -> Result<()> {
	create_folders()?;
	validate_zip_file(path)?;

	Ok(())
}

fn create_folders() -> Result<()> {
	create_if_not_exists("data")?;
	create_if_not_exists("data/zips")?;
	create_if_not_exists("data/imports")?;
	Ok(())
}

fn create_if_not_exists(path: &str) -> Result<()> {
	let path = PathBuf::from(path);
	if !path.exists() { fs::create_dir(path)?; }
	Ok(())
}

fn validate_zip_file(path: PathBuf) -> Result<()> {
	if !path.exists() { return Err(anyhow!("File not found: {:?}", path)); }
	if !path.is_file() { return Err(anyhow!("Not a file: {:?}", path)); }

	let extension = path.extension().unwrap_or_default();
	if extension != "zip" { return Err(anyhow!("Not a ZIP file: {:?}", path)); }

	let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
	let file_name_valid = Regex::new(ZIP_PATTERN).unwrap().is_match(file_name);
	if !file_name_valid { return Err(anyhow!("Invalid ZIP file name: {file_name}\nExample file name: {}", EXAMPLE_FILE_NAME)); }

	Ok(())
}
