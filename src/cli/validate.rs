// ==== src/cli/validate.rs ====
//! Validation and normalization: RawArgs -> ParseOutcome.

use super::config::{CliConfig, Command, Format, NamingMode};
use super::raw::RawArgs;

/// Outcome of parsing and validation.
#[derive(Debug)]
pub enum ParseOutcome {
    Run(CliConfig),
    ShowHelp,
    ShowVersion,
}

/// Convert raw args to validated outcome.
pub fn normalize(raw: RawArgs) -> Result<ParseOutcome, String> {
    // Early exits for help/version
    if raw.help {
        return Ok(ParseOutcome::ShowHelp);
    }
    if raw.version {
        return Ok(ParseOutcome::ShowVersion);
    }

    // No command and no flags: show help
    if raw.command.is_none()
        && !raw.webp
        && !raw.png
        && !raw.jpg
        && raw.naming.is_none()
        && raw.output.is_none()
    {
        return Ok(ParseOutcome::ShowHelp);
    }

    // Require exactly one command
    let cmd_str = raw
        .command
        .ok_or_else(|| "Missing command: use 'full' or 'box'".to_string())?;
    let command = match cmd_str.as_str() {
        "full" => Command::Full,
        "box" => Command::Box,
        other => return Err(format!("Unknown command: '{}'. Use 'full' or 'box'.", other)),
    };

    // Format flags mutual exclusivity
    let format_flags = [raw.webp, raw.png, raw.jpg];
    let format_flag_count = format_flags.iter().filter(|&&b| b).count();
    if format_flag_count > 1 {
        return Err(
            "Conflicting format flags: use at most one of --webp, --png, --jpg".to_string(),
        );
    }

    // Naming mode parsing
    let naming = match raw.naming.as_deref() {
        None | Some("timestamp") => NamingMode::Timestamp,
        Some("incremental") => NamingMode::Incremental,
        Some("hash") => NamingMode::Hash,
        Some(other) => {
            return Err(format!(
                "Invalid --naming mode: '{}'. Use: timestamp, incremental, hash",
                other
            ))
        }
    };

    // Determine output path and format
    let (output, format) = match raw.output {
        Some(path_str) => {
            // With -o: format flags are illegal; infer from extension
            if format_flag_count > 0 {
                return Err(
                    "Format flags (--webp, --png, --jpg) cannot be used with --output".to_string(),
                );
            }
            let path = std::path::PathBuf::from(path_str);
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            let inferred = match ext.as_str() {
                "png" => Format::Png,
                "jpg" | "jpeg" => Format::Jpg,
                "webp" => Format::Webp,
                _ => Format::Webp, // default when extension unrecognized
            };
            (Some(path), inferred)
        }
        None => {
            // Without -o: use format flag or default WebP
            let fmt = if raw.png {
                Format::Png
            } else if raw.jpg {
                Format::Jpg
            } else {
                Format::Webp // --webp or default
            };
            (None, fmt)
        }
    };

    Ok(ParseOutcome::Run(CliConfig {
        command,
        output,
        format,
        naming,
    }))
}
