# Marklip Specification

## Purpose
Marklip is a macOS CLI utility that converts clipboard contents between Markdown and HTML without using stdin/stdout. It targets Apple Silicon machines.

## Commands
- `marklip to-html` – converts Markdown text clipboard content to HTML and stores the result back into the clipboard as both HTML and plain text (Markdown stripped to plain text).
- `marklip to-md` – converts HTML clipboard content to Markdown text and stores the result back into the clipboard as plain text only.

## Clipboard requirements
- `to-html`: fails if the clipboard lacks plain text (`NSPasteboard.PasteboardType.string`).
- `to-md`: ignores plain text and fails if the clipboard lacks HTML (`NSPasteboard.PasteboardType.html`).
- UTF-8 is assumed for all clipboard text encodings.

## Success behavior
- Clears existing clipboard data, then writes the converted representation:
  - `to-html`: writes HTML plus a plain-text rendition of the original Markdown (stripped).
  - `to-md`: writes plain text only.
- Reports a success message (stderr or Notification Center). Exit status 0.

## Error handling
- Exit 1 when the required clipboard format is missing.
- Exit 2 on conversion failures.
- Exit 255 for any other errors.
- Error messages are emitted to stderr unless `--notify` is used.

## Options
- `-h, --help`      Show help text.
- `-v, --version`   Show version.
- `-q, --quiet`     Suppress stderr output.
- `-n, --notify`    Send messages via Notification Center instead of stderr. If notification dispatch fails, fall back to stderr (even with `--quiet`). Notifications are also sent for successful conversions. When combined with `--quiet`, notifications still take precedence.

## Inputs / Outputs
- Input and output are clipboard only; there is no stdin or stdout data flow.

## Localization
- All user-visible messages are in English.

## Platform
- macOS, Apple Silicon only.

## Usage examples
```shell
marklip to-html
marklip to-md --notify
marklip to-html --quiet
```
