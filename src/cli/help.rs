/// Show full help text to stdout.
pub fn show_help() {
    println!(r#"gnome-screenshotter — GNOME/Wayland-first CLI screenshots (WebP by default)

USAGE:
  gnome-screenshotter [OPTIONS] <COMMAND> [COMMAND OPTIONS]

COMMANDS:
  full        Capture the full screen
  box         Capture a rectangular region using the mouse (interactive portal selection)

GLOBAL OPTIONS:
  -o, --output <PATH>
          Save to an exact file path.
          Format is inferred from the file extension (.webp, .png, .jpg, .jpeg, and .avif).
          When --output is used, format flags like --webp/--png/--jpg are not allowed.
          If the file already exists, a new file will be added with the current timestamp before the extension.

      --webp
          Save using WebP format (default when --output is not provided)

      --png
          Save using PNG format (shortcut; no need to type -o shot.png)

      --jpg | --jpeg
          Save using JPEG format (shortcut; no need to type -o shot.jpg)

      --quality <0-100>
          Output quality for lossy formats (currently JPEG/WebP lossy mode).
          Ignored for PNG and lossless WebP.
          [default: 90]

      --lossless
          Use lossless encoding when supported (e.g. WebP).
          Ignored for formats that do not support lossless mode.

      --prefix <TEXT>
          File name prefix used when --output is not provided.
          [default: "Screenshot"]

      --naming <MODE> | --naming
          Auto-generated file naming strategy when --output is not provided.
          Possible values:
            timestamp      "Screenshot - 2026-02-12 11_22_33.webp" (default)
            incremental   "Screenshot - 0001.webp", "Screenshot - 0002.webp", ...
            hash          "Screenshot - a1b2c3d4.webp" (short content hash)
          [default: timestamp]

      --dir <DIR>
          Output directory for auto-generated filenames (when --output is not provided).
          If omitted, uses the current directory (or platform default if configured).

      --print-path
          Print the final saved file path to stdout on success (script-friendly)

  -q, --quiet
          Suppress non-error output

  -h, --help
          Print help

  -V, --version
          Print version

FORMAT SHORTCUTS (when NOT using --output):
  By default, screenshots are saved as WebP.
  Use one of these for quick format selection:
    --webp   Save as .webp (default)
    --png    Save as .png
    --jpg    Save as .jpg

NAMING EXAMPLES (auto-generated names):
  Default (timestamp):
    Screenshot - 2026-02-12 11_22_33.webp

  Custom prefix + timestamp:
    --prefix "RimWorld"
    => RimWorld - 2026-02-12 11_22_33.webp

  Incremental:
    --naming incremental
    => Screenshot - 0001.webp, Screenshot - 0002.webp, ...

  Short file hash:
    --naming hash
    => Screenshot - a1b2c3d4.webp

IMPORTANT RULES:
  • help is not a subcommand; use --help
  • If -o/--output is used, format is determined by the file extension
  • If -o/--output is used, --webp/--png/--jpg/--jpeg are invalid
  • If no format is specified and no -o is given, WebP is used by default

EXAMPLES:
  gnome-screenshotter full
      Save full screenshot as:
      "Screenshot - YYYY-MM-DD HH_MM_SS.webp"

  gnome-screenshotter box
      Interactive rectangle capture, saved as WebP by default

  gnome-screenshotter box --png
      Interactive rectangle capture, saved as PNG

  gnome-screenshotter full --jpg --quality 85
      Full screenshot as JPEG quality 85

  gnome-screenshotter full --prefix "RimWorld" --naming incremental
      Saves as "RimWorld - 0001.webp", etc.

  gnome-screenshotter box --naming hash --print-path
      Saves as "Screenshot - <hash>.webp" and prints the path

  gnome-screenshotter full -o ~/Pictures/shot.png
      Explicit path; format inferred from ".png"
"#);
}
