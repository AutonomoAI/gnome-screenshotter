// ==== src/cli/mod.rs ====
//! Custom CLI parser for gnome-screenshotter (Autonomo-style).
//! No clap. Uses lexopt for token scanning only.

mod config;
mod help;
mod raw;
mod validate;

pub use config::{CliConfig, Command, Format, NamingMode};
pub use validate::ParseOutcome;

use std::env;

/// Entry point: parse argv and produce outcome.
pub fn parse() -> Result<ParseOutcome, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let raw = raw::scan(&args)?;
    validate::normalize(raw)
}

/// Show version to stdout.
pub fn show_version() {
    println!("gnome-screenshotter {}", env!("CARGO_PKG_VERSION"));
}

pub use help::show_help;
