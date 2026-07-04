mod subcommands;

use std::{ops::ControlFlow, process::ExitCode};
use clap::{Parser, Subcommand};

use subcommands::{init_command, ls_command, add_command, remove_command, send_command, check_rivet::check_rivet_folder};

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
    #[command(visible_alias = "a")]
    Add {
        /// Name of your request
        #[arg(short, long)]
        name: String,

        /// Collection where you want to add the request in
        #[arg(short, long)]
        collection: String
    },

    /// Removes a request in a saved collection
    #[command(visible_alias = "rm")]
    Remove {
        /// Name of your request to be deleted
        #[arg(short, long)]
        name: String,

        /// Collection where you want to remove the request from
        #[arg(short, long)]
        collection: String
    },

    /// Sends your saved request
    #[command(visible_alias = "s")]
    Send {
        /// Name of your request to be used
        #[arg(short, long)]
        name: String,

        /// Collection where the request is in
        #[arg(short, long)]
        collection: String
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match &cli.cmd {
        Commands::Init => {
            init_command::init_function()
        },

        Commands::Ls => {
            if let ControlFlow::Break(_) = check_rivet_folder() {
                return ExitCode::FAILURE;
            }

            ls_command::ls_function()
        },

        Commands::Add { name, collection } => {
            if let ControlFlow::Break(_) = check_rivet_folder() {
                return ExitCode::FAILURE;
            }

            add_command::add_function(name, collection)
        }

        Commands::Remove { name, collection } => {
            if let ControlFlow::Break(_) = check_rivet_folder() {
                return ExitCode::FAILURE;
            }

            remove_command::remove_function(name, collection)
        }

        Commands::Send { name, collection } => {
            if let ControlFlow::Break(_) = check_rivet_folder() {
                return ExitCode::FAILURE;
            }

            send_command::send_function(name, collection)
        }

    };

    if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
