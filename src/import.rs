use std::{fs, path::PathBuf};
use anyhow::{anyhow, Result};
use regex::Regex;
use zip;

const DIR: &str = ".";
const ZIP_PATTERN: &str = r#"instagram-.+?-\d{4}-\d{2}-\d{2}-\w{8}\.zip"#;
const EXAMPLE_FILE_NAME: &str = "instagram-my_username-2021-01-01-aBcD1X2Y.zip";

mod paths {
	pub const DATA: &str = "data";
	pub const TMP: &str = "tmp_data";
	pub const ZIPS: &str = "data/zips";
	pub const IMPORTS: &str = "data/imports";
}

use paths::*;

pub fn import_zip(path: PathBuf) -> Result<()> {
	create_all_folders()?;
	validate_file(&path)?;
	extract_zip(path)?;

	Ok(())
}

fn create_all_folders() -> Result<()> {
	fs::remove_dir_all(TMP).ok();
	create_folder(TMP)?;
	create_folder(DATA)?;
	create_folder(ZIPS)?;
	create_folder(IMPORTS)?;
	Ok(())
}

fn create_folder(path: &str) -> Result<()> {
	let path = PathBuf::from(DIR).join(path);
	if !path.exists() { fs::create_dir(path)?; }
	Ok(())
}

fn validate_file(path: &PathBuf) -> Result<()> {
	if !path.exists() { return Err(anyhow!("File not found: {:?}", path)); }
	if !path.is_file() { return Err(anyhow!("Not a file: {:?}", path)); }

	let extension = path.extension().unwrap_or_default();
	if extension != "zip" { return Err(anyhow!("Not a ZIP file: {:?}", path)); }

	let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
	let file_name_valid = Regex::new(ZIP_PATTERN).unwrap().is_match(file_name);
	if !file_name_valid { return Err(anyhow!("Invalid ZIP file name: {file_name}\nExample file name: {}", EXAMPLE_FILE_NAME)); }

	Ok(())
}

fn extract_zip(path: PathBuf) -> Result<()> {
	let file_name = path.file_name().unwrap_or_default();
	let destination = PathBuf::from(TMP).join(file_name);
	fs::copy(&path, &destination)?;

	let file = fs::File::open(destination)?;
	let reader = std::io::BufReader::new(file);
	let mut archive = zip::ZipArchive::new(reader)?;
	archive.extract(TMP)?;

	validate_extract()?;

	Ok(())
}

fn validate_extract() -> Result<()> {
	// continue

	Ok(())
}
