use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::{env, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(
    author = "Sakthi-dev-tech",
    version,
    about,
    about = "A CLI tool for your cURL commands"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Init a .rivet folder in your current folder
    Init {
        /// Init .rivet folder in a custom path
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Init { path } => {
            let current_path = match env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    panic!("Error getting current directory: {}", err.red());
                }
            };

            let rivet_path = match path {
                Some(path) => current_path.join(path),
                None => current_path
            };
            let rivet_path = rivet_path.join(".rivet");

            let dirs = [
                rivet_path.join("requests/"),
                rivet_path.join("history/"),
            ];

            let files = [
                rivet_path.join("config.toml")
            ];

            for dir in dirs {
                if let Err(err) = fs::create_dir_all(&dir) {
                    panic!("Error creating {}: {}", dir.display(), err.bright_red());
                }
            }

            for file in files {
                if !file.exists() {
                    fs::write(&file, "")
                        .expect("Error creating file!");
                }
            }

            println!("{}", "Initialised successfully!".green())
        }
    }
}
