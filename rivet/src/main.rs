mod actions;
mod clicommands;
mod types;
mod tui;

use std::io;

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

use actions::{add_action, check_rivet::check_rivet_folder, init_action, remove_action};
use clicommands::{ls_command, send_command};

#[derive(Parser, Debug)]
#[command(
    author = "Sakthi-dev-tech",
    version,
    about,
    about = "A CLI tool for your cURL commands"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
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
        /// Request path in collection/name format
        path: String,
    },

    /// Removes a request in a saved collection
    #[command(visible_alias = "rm")]
    Remove {
        /// Request path in collection/name format
        path: String,
    },

    /// Sends your saved request
    #[command(visible_alias = "s")]
    Send {
        /// Request path in collection/name format
        path: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let print_error = |error: String| println!("{}", error.red());

    // this is for cli commands as it has a subcommand
    if let Some(command) = &cli.cmd {
        let result = match command {
            Commands::Init => init_action::init_function().map_err(print_error),

            Commands::Ls => check_rivet_folder()
                .map_err(print_error)
                .and_then(|_| ls_command::ls_function()),

            Commands::Add { path } => check_rivet_folder()
                .map_err(print_error)
                .and_then(|_| add_action::add_function(path).map_err(print_error)),

            Commands::Remove { path } => check_rivet_folder()
                .map_err(print_error)
                .and_then(|_| remove_action::remove_function(path).map_err(print_error)),

            Commands::Send { path } => check_rivet_folder()
                .map_err(print_error)
                .and_then(|_| send_command::get_response_table(path).map_err(|_| ())),
        };

        match result {
            Ok(()) => Ok(()),
            Err(()) => Err(io::Error::other("Invalid Rivet Command!")),
        }
    } else {
        println!("TUI Called!");
        Ok(())
    }
}
