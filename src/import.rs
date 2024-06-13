use std::{fs, path::PathBuf};
use anyhow::{anyhow, Result};
use regex::Regex;
use zip;

const DIR: &str = ".";
const ZIP_PATTERN: &str = r#"instagram-.+?-\d{4}-\d{2}-\d{2}-\w{8}\.zip"#;
const EXAMPLE_FILE_NAME: &str = "instagram-my_username-2021-01-01-aBcD1X2Y.zip";

struct Paths {
	data: String,
	extract: String,
	zips: String,
	imports: String,
	connections: String,
	follower_data: String,
}

impl Paths {
	fn new(dir: &str) -> Self {
		let data = format!("{dir}/data");
		let zips = format!("{data}/zips");
		let imports = format!("{data}/imports");

		let extract = format!("{dir}/extract");
		let connections = format!("{extract}/connections");
		let follower_data = format!("{connections}/followers_and_following");

		Self {
			data,
			extract,
			zips,
			imports,
			connections,
			follower_data
		}
	}
}

pub fn import_zip(path: PathBuf) -> Result<()> {
	let paths = Paths::new(DIR);

	create_all_folders(&paths)?;
	validate_zip(&path)?;
	extract_zip(&path, &paths.extract)?;
	validate_extract(&paths)?;

	Ok(())
}

fn create_all_folders(paths: &Paths) -> Result<()> {
	fs::remove_dir_all(&paths.extract).ok();
	create_folder(&paths.extract)?;
	create_folder(&paths.data)?;
	create_folder(&paths.zips)?;
	create_folder(&paths.imports)?;
	Ok(())
}

fn create_folder(path: &str) -> Result<()> {
	let path = PathBuf::from(DIR).join(path);
	if !path.exists() { fs::create_dir(path)?; }
	Ok(())
}

fn validate_zip(path: &PathBuf) -> Result<()> {
	if !path.exists() { return Err(anyhow!("File not found: {:?}", path)); }
	if !path.is_file() { return Err(anyhow!("Not a file: {:?}", path)); }

	let extension = path.extension().unwrap_or_default();
	if extension != "zip" { return Err(anyhow!("Not a ZIP file: {:?}", path)); }

	let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
	let file_name_valid = Regex::new(ZIP_PATTERN).unwrap().is_match(file_name);
	if !file_name_valid { return Err(anyhow!("Invalid ZIP file name: {file_name}\nExample file name: {}", EXAMPLE_FILE_NAME)); }

	Ok(())
}

fn extract_zip(path: PathBuf, target_path: &str) -> Result<()> {
	let file_name = path.file_name().unwrap_or_default();
	let destination = PathBuf::from(target_path).join(file_name);
	fs::copy(&path, &destination)?;

	let file = fs::File::open(destination)?;
	let reader = std::io::BufReader::new(file);
	let mut archive = zip::ZipArchive::new(reader)?;
	archive.extract(target_path)?;

	Ok(())
}

fn validate_extract(paths: &Paths) -> Result<()> {
	if !PathBuf::from(&paths.connections).exists() { return Err(anyhow!("Connections folder not found")); }
	if !PathBuf::from(&paths.follower_data).exists() { return Err(anyhow!("followers_and_following folder not found")); }
	Ok(())
}
