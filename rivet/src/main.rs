mod subcommands;

use std::ops::ControlFlow;
use clap::{Parser, Subcommand};

use subcommands::{init_command, ls_command};

#[derive(Parser, Debug)]
#[command(
    author = "Sakthi-dev-tech",
    version,
    about,
    about = "A CLI tool for your cURL commands"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Init a .rivet folder in your current folder
    Init,
    /// List all your saved requests
    Ls
}

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Init => {
            init_command::init_function();
        },

        Commands::Ls => {
            if let ControlFlow::Break(_) = ls_command::ls_function() {
                return;
            }
        }
    }
}
