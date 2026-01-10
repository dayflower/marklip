# Raycast Script Commands for marklip

These scripts let Raycast trigger `marklip` conversions on the clipboard. All user-facing messages stay in English to match `marklip` conventions.

## Prerequisites

- Install `marklip` and ensure the binary is on your `PATH` (macOS users can run `brew install dayflower/tap/marklip`, or download a binary from GitHub Releases).
- Copy the scripts into your Raycast Script Commands directory (Raycast > Settings > Extensions > Scripts > `Open Script Folder`) and make them executable: `chmod +x marklip-*.sh`.

## Available scripts

- `marklip-auto.sh` — Detects clipboard content and converts Markdown ↔ HTML automatically, falling back to notifications for results or errors.
- `marklip-to-html.sh` — Converts clipboard Markdown to HTML (and plain text) using `marklip to-html --notify`.
- `marklip-to-md.sh` — Converts clipboard HTML to Markdown using `marklip to-md --notify`.

## Usage

1. Add the scripts to the Raycast Script Commands folder and ensure execute permission.
2. In Raycast, search for the script title (e.g., “Convert clipboard Markdown -> HTML (to-html)”) and run it.
3. Check notifications for success or error details; if notifications fail, output appears in the Raycast command panel.

## Customization

### Notification Behavior

All example scripts use the `--notify` flag to send error messages to Notification Center instead of showing them in Raycast. This provides a better user experience when running commands from Raycast.

If you prefer to see all output in Raycast, remove the `--notify` flag and change `@raycast.mode` from `silent` to `fullOutput`:

```bash
# Before
# @raycast.mode silent
marklip auto --notify

# After
# @raycast.mode fullOutput
marklip auto
```

## Keyboard Shortcuts

You can assign keyboard shortcuts to these commands in Raycast:

1. Open Raycast and search for the command
2. Press ⌘K to open actions
3. Select "Assign Keyboard Shortcut"
4. Press your desired key combination
