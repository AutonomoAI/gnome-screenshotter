// ==== src/error.rs ====
//! Project-wide error types.

use std::fmt;

/// Errors originating from CLI parsing or validation.
#[derive(Debug)]
pub enum CliError {
    InvalidCommand(String),
    MissingCommand,
    ConflictingFormatFlags,
    FormatFlagWithOutput,
    InvalidNamingMode(String),
    UnexpectedArgument(String),
    UnknownFlag(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::InvalidCommand(s) => write!(f, "Unknown command: '{}'", s),
            CliError::MissingCommand => write!(f, "Missing command: use 'full' or 'box'"),
            CliError::ConflictingFormatFlags => {
                write!(f, "Conflicting format flags: use at most one of --webp, --png, --jpg")
            }
            CliError::FormatFlagWithOutput => {
                write!(f, "Format flags cannot be used with --output")
            }
            CliError::InvalidNamingMode(s) => {
                write!(f, "Invalid --naming mode: '{}'. Use: timestamp, incremental, hash", s)
            }
            CliError::UnexpectedArgument(s) => write!(f, "Unexpected argument: {}", s),
            CliError::UnknownFlag(s) => write!(f, "Unknown argument: {}", s),
        }
    }
}

impl std::error::Error for CliError {}

/// General application error.
#[derive(Debug)]
pub enum AppError {
    Cli(CliError),
    Io(std::io::Error),
    Portal(String),
    Image(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Cli(e) => write!(f, "{}", e),
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::Portal(s) => write!(f, "Portal error: {}", s),
            AppError::Image(s) => write!(f, "Image error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

impl From<CliError> for AppError {
    fn from(e: CliError) -> Self {
        AppError::Cli(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}
