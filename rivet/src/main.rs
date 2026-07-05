mod actions;
mod subcommands;
mod types;

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::process::ExitCode;

use actions::{add_action, check_rivet::check_rivet_folder, init_action, remove_action};
use subcommands::{ls_command, send_command};

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
        collection: String,
    },

    /// Removes a request in a saved collection
    #[command(visible_alias = "rm")]
    Remove {
        /// Name of your request to be deleted
        #[arg(short, long)]
        name: String,

        /// Collection where you want to remove the request from
        #[arg(short, long)]
        collection: String,
    },

    /// Sends your saved request
    #[command(visible_alias = "s")]
    Send {
        /// Name of your request to be used
        #[arg(short, long)]
        name: String,

        /// Collection where the request is in
        #[arg(short, long)]
        collection: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let print_error = |error: String| println!("{}", error.red());

    let result = match &cli.cmd {
        Commands::Init => init_action::init_function().map_err(print_error),

        Commands::Ls => check_rivet_folder()
            .map_err(print_error)
            .and_then(|_| ls_command::ls_function()),

        Commands::Add { name, collection } => check_rivet_folder()
            .map_err(print_error)
            .and_then(|_| add_action::add_function(name, collection).map_err(print_error)),

        Commands::Remove { name, collection } => check_rivet_folder()
            .map_err(print_error)
            .and_then(|_| remove_action::remove_function(name, collection).map_err(print_error)),

        Commands::Send { name, collection } => check_rivet_folder()
            .map_err(print_error)
            .and_then(|_| send_command::get_response_table(name, collection).map_err(|_| ())),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
