# gnome-screenshotter

A **Rust CLI screenshot tool for GNOME/Wayland** with a clean UX:

- **`full`** → full-screen screenshots
- **`box`** → interactive rectangular selection (using the desktop/portal UI)
- **WebP by default**
- **Zero-filename mode** (auto-saves if you don’t specify `-o`)
- **Never overwrites files** (collision-safe suffixing like `.1`, `.2`, etc.)

> Built for GNOME/Wayland using **xdg-desktop-portal**, so it works with modern secure desktop environments instead of relying on old X11-style screen grabbing.

---

## Why this exists

Linux screenshot CLIs are often missing one or more of these:

- Wayland-first behavior
- simple interactive region capture from the CLI
- WebP output by default
- easy format shortcuts (`--png`, `--jpg`. `--avif`)
- safe output behavior (no accidental overwrites)

`gnome-screenshotter` is designed to make screenshots from the terminal feel **fast, obvious, and script-friendly**.

---

## Features

- ✅ **GNOME/Wayland-first** screenshot capture
- ✅ **Portal-backed** (xdg-desktop-portal / `ashpd`)
- ✅ **Full-screen** and **interactive box selection**
- ✅ **WebP by default**
- ✅ **`--png` / `--jpg` / `--webp` / `--avif` shortcuts**
- ✅ **Auto-generated filenames**
- ✅ **Naming strategies**: `timestamp`, `incremental`, `hash`
- ✅ **Never overwrite files**
- ✅ **Collision-safe output naming** (`file.png` → `file.1.png`)

---

## Commands

### `full`
Capture the full screen.

```bash
gnome-screenshotter full
````

### `box`

Capture an interactively selected rectangular region using the desktop’s built-in selection UI (portal/compositor UI).

```bash
gnome-screenshotter box
```

---

## Output behavior

### Default format: WebP

If no output path is provided, screenshots are saved as **WebP** by default.

### Default filename format

If you don’t pass `-o/--output`, the file is auto-saved using a timestamped name like:

```text
Screenshot - 2026-02-22 11_22_33.webp
```

Time uses underscores (`HH_MM_SS`) so glob/regex searches are easier and less ambiguous.

---

## File safety (never overwrite)

This tool **never overwrites files**.

If the target filename already exists, it appends/increments a suffix **before the extension**:

* `shot.png` → `shot.1.png`
* `shot.1.png` → `shot.2.png`

This applies to:

* auto-generated filenames
* explicit `-o/--output` paths
* all naming strategies

There is **no `--overwrite` option**.

---

## Format shortcuts

When no `-o/--output` path is provided, you can choose format with shortcuts:

* `--webp` (default)
* `--png`
* `--jpg`

Examples:

```bash
gnome-screenshotter full --png
gnome-screenshotter box --jpg
```

### Important rule (`-o` vs format flags)

If `-o/--output` is provided, the format is determined by the **file extension**.

So this is valid:

```bash
gnome-screenshotter full -o shot.png
```

And this is invalid (conflicting format source):

```bash
gnome-screenshotter full -o shot.png --webp
```

---

## Naming strategies (`--naming`)

Choose how auto-generated filenames are created:

* `--naming timestamp` *(default)*
* `--naming incremental`
* `--naming hash`

### Timestamp (default)

```bash
gnome-screenshotter full --naming timestamp
```

Example output:

```text
Screenshot - 2026-02-22 11_22_33.webp
```

### Incremental

```bash
gnome-screenshotter full --naming incremental
```

Example outputs:

```text
Screenshot - 0001.webp
Screenshot - 0002.webp
```

### Hash (short file hash)

```bash
gnome-screenshotter box --naming hash
```

Example output:

```text
Screenshot - a1b2c3d4.webp
```

---

## Common examples

### Full-screen screenshot (default WebP, auto-name)

```bash
gnome-screenshotter full
```

### Interactive box screenshot (default WebP, auto-name)

```bash
gnome-screenshotter box
```

### Force PNG output (auto-name)

```bash
gnome-screenshotter full --png
```

### Force JPEG output (auto-name)

```bash
gnome-screenshotter box --jpg
```

### Save to an explicit file path (format inferred from extension)

```bash
gnome-screenshotter full -o ~/Pictures/shot.png
```

### Use a custom naming strategy

```bash
gnome-screenshotter full --naming incremental
gnome-screenshotter box --naming hash
```

---

## Design notes

### Portal/compositor UI for `box`

`box` uses the desktop’s **built-in** interactive selection UI via the portal/compositor.

This project does **not** draw its own overlay or track mouse input directly.

### Wayland-first approach

On Wayland, secure screenshot capture generally requires portal/compositor APIs. `gnome-screenshotter` is built around that model from the start.

---

## Build status

This project is under active development.

MVP focus:

* `full`
* `box`
* WebP default output
* safe filename generation and collision suffixing
* clean CLI UX

---

## Planned architecture (high level)

* **Capture layer**: portal-backed screenshot adapter (`ashpd`)
* **CLI layer**: custom parser + validation (Autonomo style; not clap-based)
* **Output pipeline**:

  * format resolution
  * filename generation
  * collision-safe suffixing
  * decode/transcode/save

---

## Philosophy

This tool prioritizes:

* **Practical CLI UX**
* **Safe defaults**
* **Deterministic behavior**
* **Wayland/GNOME compatibility**
* **Script-friendly output**

No surprises. No silent overwrites. No unnecessary typing.

---

## License

TBD

---

## Author / Project

Built as part of the **Autonomo AI** ecosystem.

Repository: https://github.com/AutonomoAI/gnome-screenshotter
