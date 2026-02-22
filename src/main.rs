// ==== src/main.rs ====
//! Entry point for gnome-screenshotter.

mod cli;
mod error;

use cli::{show_help, show_version, ParseOutcome};

fn main() {
   match cli::parse() {
       Ok(ParseOutcome::ShowHelp) => {
           show_help();
       }
       Ok(ParseOutcome::ShowVersion) => {
           show_version();
       }
       Ok(ParseOutcome::Run(config)) => {
           println!("Running with config: {:?}", config);
           // TODO: dispatch to capture module
       }
       Err(e) => {
           eprintln!("Error: {}", e);
           std::process::exit(1);
       }
   }
}
