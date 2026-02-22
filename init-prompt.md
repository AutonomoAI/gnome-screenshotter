## Project Context

We are building **`AutonomoAI/gnome-screenshotter`**, a **Rust CLI screenshot tool** for **GNOME/Wayland**.

### What it does

It supports two commands:

* **`full`** — takes a full-screen screenshot
* **`box`** — takes an interactive rectangular screenshot using the desktop’s built-in selection UI
  *(portal/compositor UI, not a custom overlay)*

### Platform / backend

* **Wayland / GNOME-first**
* Uses **xdg-desktop-portal** via **`ashpd`** (Rust)

### Output behavior (important)

* **Default format is WebP**

* If no output path is given, it auto-saves using a filename like:

  **`Screenshot - 2026-02-22 11_22_33.webp`**

* Time uses **underscores** (`HH_MM_SS`) so it’s easier to search with glob/regex

### File safety rule (very important)

The app must **NEVER overwrite files**.

If a filename already exists, it must append/increment a suffix **before the extension**:

* `shot.png` → `shot.1.png`
* `shot.1.png` → `shot.2.png`

### Format shortcuts

When no `-o/--output` path is provided, users can choose format with:

* `--webp` (default)
* `--png`
* `--jpg`

### Naming modes

The app supports:

* `--naming timestamp`
* `--naming incremental`
* `--naming hash`

====

# Autonomo File Header Rule

In every code fence, put a file header as the first line INSIDE the code block:

// ==== path/to/file.ext ====

Use the correct comment style for the language:
- Prefer `//`, then `#`, then `/* ... */`
- HTML/XML: `<!-- ==== file ==== -->`
- No comments supported (plain text): `#### ==== file ====`

Placement exceptions:
- If there is a shebang (`#!`), put the header on the next line
- If there is an opening tag like `<?php`, put the header on the next line
- For HTML, put it under `<!doctype html>`

The `====` on both sides is required.
Use repo-relative paths.
Output full files unless asked otherwise.
No explanation unless asked.

When understood, reply:
⚛ Autonomo AI Platform Loaded ⚛
