mod subcommands;

use std::ops::ControlFlow;
use clap::{Parser, Subcommand};

use subcommands::{init_command, ls_command, add_command};

use crate::subcommands::check_rivet::check_rivet_folder;

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
    Ls,
    /// Adds a new request in a saved collection
    Add {
        /// Name of your request
        #[arg(short, long)]
        name: String,

        /// Collection where you want to add the request in
        #[arg(short, long)]
        collection: String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Init => {
            init_command::init_function();
        },

        Commands::Ls => {
            if let ControlFlow::Break(_) = check_rivet_folder() {
                return;
            }

            ls_command::ls_function();
        },

        Commands::Add { name, collection } => {
            if let ControlFlow::Break(_) = check_rivet_folder() {
                return;
            }

            add_command::add_function(name, collection);
        }
    }
}

