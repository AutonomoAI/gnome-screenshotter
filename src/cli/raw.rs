// ==== src/cli/raw.rs ====
//! Raw argv scanning with lexopt. No validation here—just token collection.

use lexopt::{Arg, ValueExt};

/// Unvalidated collection of CLI tokens.
#[derive(Debug, Default)]
pub struct RawArgs {
    pub command: Option<String>,
    pub output: Option<String>,
    pub webp: bool,
    pub png: bool,
    pub jpg: bool,
    pub avif: bool,
    pub naming: Option<String>,
    pub help: bool,
    pub version: bool,
}

/// Scan argv into RawArgs. Returns Err(String) for lexopt-level issues.
pub fn scan(args: &[String]) -> Result<RawArgs, String> {
    let mut raw = RawArgs::default();
    let mut parser = lexopt::Parser::from_args(args.iter().cloned());

    while let Some(arg) = parser.next().map_err(|e| e.to_string())? {
        match arg {
            Arg::Short('h') | Arg::Long("help") => raw.help = true,
            Arg::Short('V') | Arg::Long("version") => raw.version = true,
            Arg::Short('o') | Arg::Long("output") => {
                let val = parser.value().map_err(|e| e.to_string())?;
                raw.output = Some(val.string().map_err(|e| e.to_string())?);
            }
            Arg::Long("webp") => raw.webp = true,
            Arg::Long("png") => raw.png = true,
            Arg::Long("jpg") => raw.jpg = true,
            Arg::Long("avif") => raw.avif = true,
            Arg::Long("naming") => {
                let val = parser.value().map_err(|e| e.to_string())?;
                raw.naming = Some(val.string().map_err(|e| e.to_string())?);
            }
            Arg::Value(val) => {
                if raw.command.is_none() {
                    raw.command = Some(val.string().map_err(|e| e.to_string())?);
                } else {
                    return Err(format!(
                        "Unexpected argument: {}",
                        val.string().map_err(|e| e.to_string())?
                    ));
                }
            }
            _ => return Err(format!("Unknown argument: {}", arg.unexpected())),
        }
    }

    Ok(raw)
}
