// ==== src/cli/config.rs ====
//! Validated CLI configuration types.

/// Screenshot command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Full,
    Box,
}

/// Output image format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Webp,
    Png,
    Jpg,
}

/// Auto-naming strategy when no output path given.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingMode {
    Timestamp,
    Incremental,
    Hash,
}

/// Fully validated and normalized CLI configuration.
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub command: Command,
    pub output: Option<std::path::PathBuf>,
    pub format: Format,
    pub naming: NamingMode,
}
