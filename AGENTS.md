# Agent Guide for marklip

## Scope and intent

- marklip is a cross-platform CLI tool that converts clipboard content between Markdown and HTML via subcommands `to-html` and `to-md`; it has no stdin/stdout I/O path.
- Primary development happens on macOS, but the tool is intended to run anywhere `clipboard-rs` and `notify-rust` have working backends (macOS, Linux, Windows).

## Error and messaging rules

- Exit codes: 1 when the required clipboard format is missing, 2 on conversion failure, 255 for all other errors.
- All user-visible messages must be English. Respect `--quiet` and `--notify` precedence (notifications win over quiet; on notification failure, fall back to stderr).

## Tech stack

- Rust; use `clipboard-rs` for clipboard access, `markdown` for Markdown→HTML, `strip_markdown` to derive plain text from Markdown, `htmd` for HTML→Markdown, and `notify-rust` (platform backends; mac uses `mac-notification-sys`).
- Clipboard encoding is assumed to be UTF-8.

## Contribution notes

- Preserve the clipboard-only contract (no stdio data paths).
- Clear the clipboard before writing converted output to avoid mixed formats.
- `to-html` must write both HTML and plain text (the Markdown stripped via `strip_markdown`) to the clipboard so consumers can pick either flavor.
- When adding tests, prefer mocked clipboard interactions.
