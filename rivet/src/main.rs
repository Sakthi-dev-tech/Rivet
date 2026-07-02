mod subcommands;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::subcommands::init_function;

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
            init_function(path);
        }
    }
}

