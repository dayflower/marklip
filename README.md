# marklip

A lightning-fast clipboard utility for seamless Markdown ⇄ HTML conversion.

Copy Markdown, convert to HTML with one command. Copy HTML, convert back to Markdown just as easily. No files, no pipes—just your clipboard.

## Why marklip?

When you're writing in Markdown but need to paste into rich text editors, email clients, or CMSs, you're constantly context-switching. marklip eliminates that friction:

- **One command**: `marklip auto`, `marklip to-html`, or `marklip to-md`
- **Clipboard-native**: No intermediate files or stdin/stdout juggling
- **Fast**: Built in Rust with native clipboard backends
- **Scriptable**: Clean exit codes for automation

Perfect for technical writers, developers, and anyone who lives in Markdown but works across multiple platforms.

## Quick Start

```bash
# Install via Homebrew (macOS)
brew install dayflower/tap/marklip

# Or download a release binary (macOS/Linux/Windows)
# [Releases page](https://github.com/dayflower/marklip/releases/latest)

# Or install from source
git clone https://github.com/dayflower/marklip.git
cd marklip && cargo install --path .

# Copy some Markdown, then:
marklip to-html

# Copy some HTML, then:
marklip to-md --notify

# Let marklip decide based on clipboard:
marklip auto
```

## Usage Examples

### Basic Conversion

```bash
# Write Markdown in your editor → Copy → Convert
marklip to-html

# Now paste rich text into Gmail, Notion, etc.
```

### With Notifications

```bash
# Get visual feedback when conversion completes
marklip to-md --notify
```

## Installation

### Requirements

- Rust 1.84+ (for building from source)
- A supported clipboard and notification backend (`clipboard-rs`, `notify-rust`) on your platform (macOS, Linux, Windows)

### Homebrew (macOS)

```bash
brew install dayflower/tap/marklip
```

### Launcher (macOS)

If you prefer a status bar launcher, install [marklip-launcher](https://github.com/dayflower/marklip-launcher):

- via Homebrew: `brew install dayflower/tap/marklip-launcher`

### Download Binaries (GitHub Releases)

Grab the latest tarball/zip for your platform from the [Releases page](https://github.com/dayflower/marklip/releases/latest), extract, and place `marklip` on your `PATH`.

### From Source

```bash
git clone https://github.com/dayflower/marklip.git
cd marklip
cargo install --path .
```

## Features

- **Three commands**: `auto`, `to-html`, and `to-md`
- **Clipboard-only workflow**: Reads from clipboard, writes back converted result
- **Clean output**: Clears clipboard before writing to avoid format mixing
- **Optional notifications**: Visual feedback via Notification Center
- **Quiet mode**: Perfect for scripting
- **Reliable exit codes**: Easy integration with shell scripts

## Usage

### Commands

#### `marklip auto`

Automatically converts based on clipboard content:

- If HTML is present, converts to Markdown (same behavior as `to-md`).
- Else if plain text is present and not empty, converts to HTML (same as `to-html`).
- Otherwise, returns an error.

Uses the same options and exit codes as the explicit commands.

#### `marklip to-html`

Converts Markdown (plain text) to HTML.

**Options:**

- `-q, --quiet`: Suppress stderr
- `-n, --notify`: Show Notification Center alert

**Exit codes:**

- `0`: Success
- `1`: No Markdown text in clipboard
- `2`: Conversion failed

#### `marklip to-md`

Converts HTML to Markdown.

Same options and exit codes as `to-html`.

### Global Options

- `-h, --help`: Show help
- `-v, --version`: Show version

### Example Integrations

- Raycast extension example: see `examples/raycast/README.md` for setup and usage details.

## Troubleshooting

**Q: "Required clipboard format missing" error**  
A: Make sure you've copied the correct format—plain text for `to-html`, HTML for `to-md`.

**Q: Notification doesn't appear**  
A: Check System Settings → Notifications and ensure Terminal (or your shell) has notification permissions.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

Built with:

- [pulldown-cmark](https://crates.io/crates/pulldown-cmark) for Markdown parsing
- [html2md](https://crates.io/crates/html2md) for HTML conversion
- [clipboard-rs](https://crates.io/crates/clipboard-rs) for clipboard access
