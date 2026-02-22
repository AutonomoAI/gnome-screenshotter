// ==== src/cli/mod.rs ====
//! Custom CLI parser for gnome-screenshotter (Autonomo-style).
//! No clap. Uses lexopt for token scanning only.

mod config;
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

/// Show help text to stdout.
pub fn show_help() {
    println!(
        r#"gnome-screenshotter {}

Take screenshots on GNOME/Wayland using xdg-desktop-portal.

USAGE:
    gnome-screenshotter [OPTIONS] <COMMAND>

COMMANDS:
    full    Take a full-screen screenshot
    box     Take an interactive rectangular screenshot

OPTIONS:
    -h, --help              Print help
    -V, --version           Print version
    -o, --output <PATH>     Save to specific path (infers format from extension)
    --webp                  Use WebP format (default, only without -o)
    --png                   Use PNG format (only without -o)
    --jpg                   Use JPEG format (only without -o)
    --naming <MODE>         Naming mode for auto filenames: timestamp, incremental, hash

RULES:
    - Default format is WebP when no -o is given
    - --webp, --png, --jpg are mutually exclusive
    - --webp, --png, --jpg are invalid when -o is used
    - Filenames never overwrite; collisions append .1, .2, etc. before extension
"#,
        env!("CARGO_PKG_VERSION")
    );
}

/// Show version to stdout.
pub fn show_version() {
    println!("gnome-screenshotter {}", env!("CARGO_PKG_VERSION"));
}
