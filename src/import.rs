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

pub fn import_zip(src_path: PathBuf) -> Result<()> {
	let paths = Paths::new(DIR);

	create_all_folders(&paths)?;
	validate_zip(&src_path)?;
	extract_zip(&src_path, &paths.extract)?;
	validate_extract(&paths)?;

	let file_name = src_path.file_name().unwrap();
	let dst_zip_dir = PathBuf::from(&paths.zips).join(file_name);
	fs::copy(&src_path, &dst_zip_dir)?;

	let file_stem = &src_path.file_stem().unwrap();
	let dst_imports_dir = PathBuf::from(&paths.imports).join(file_stem);
	fs::create_dir(&dst_imports_dir)?;
	
	for entry in fs::read_dir(&paths.follower_data)? {
		let entry = entry?;
		let dst_path = dst_imports_dir.join(entry.file_name());
		fs::copy(&entry.path(), dst_path)?;
	};

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

fn validate_zip(src_path: &PathBuf) -> Result<()> {
	if !src_path.exists() { return Err(anyhow!("File not found: {:?}", src_path)); }
	if !src_path.is_file() { return Err(anyhow!("Not a file: {:?}", src_path)); }

	let extension = src_path.extension().unwrap_or_default();
	if extension != "zip" { return Err(anyhow!("Not a ZIP file: {:?}", src_path)); }

	let file_name = src_path.file_name().unwrap_or_default().to_str().unwrap_or_default();
	let file_name_valid = Regex::new(ZIP_PATTERN).unwrap().is_match(file_name);
	if !file_name_valid { return Err(anyhow!("Invalid ZIP file name: {file_name}\nExample file name: {}", EXAMPLE_FILE_NAME)); }

	Ok(())
}

fn extract_zip(src_path: &PathBuf, extracts_dir: &str) -> Result<()> {
	let file_name = src_path.file_name().unwrap();
	let dst_path = PathBuf::from(extracts_dir).join(file_name);
	fs::copy(&src_path, &dst_path)?;

	let file = fs::File::open(dst_path)?;
	let reader = std::io::BufReader::new(file);
	let mut archive = zip::ZipArchive::new(reader)?;
	archive.extract(extracts_dir)?;

	Ok(())
}

fn validate_extract(paths: &Paths) -> Result<()> {
	if !PathBuf::from(&paths.connections).exists() { return Err(anyhow!("Connections folder not found")); }
	if !PathBuf::from(&paths.follower_data).exists() { return Err(anyhow!("followers_and_following folder not found")); }
	Ok(())
}
