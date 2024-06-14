use std::path::PathBuf;
use anyhow::Result;
mod import;
use clap::{Parser, Subcommand};
mod parser;

fn main() {
	match execute() {
		Ok(_) => {},
		Err(e) => println!("<!>\n{:?}", e)
	}
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
	#[command(subcommand)]
	pub command: Command
}

#[derive(Subcommand)]
enum Command {
	#[clap(name = "import", about = "import a ZIP file")]
	Import(ImportArgs)
}

#[derive(Parser)]
struct ImportArgs {
	#[arg(required = true)]
	path: PathBuf
}

fn execute() -> Result<()> {
	let cli = Cli::try_parse()?;

	match cli.command {
		Command::Import(args) => {
			import::import_zip(args.path)?;
		}
	}

	Ok(())
}
