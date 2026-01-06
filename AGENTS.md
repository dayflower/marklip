# Agent Guide for marklip

## Scope and intent

- marklip is a macOS CLI tool that converts clipboard content between Markdown and HTML via subcommands `to-html` and `to-md`; it has no stdin/stdout I/O path.
- Target platform is Apple Silicon macOS only.

## Error and messaging rules

- Exit codes: 1 when the required clipboard format is missing, 2 on conversion failure, 99 for all other errors.
- All user-visible messages must be English. Respect `--quiet` and `--notify` precedence (notifications win over quiet; on notification failure, fall back to stderr).

## Tech stack

- Rust; use `clipboard-rs` for clipboard access, `markdown` for Markdown→HTML, `strip_markdown` to derive plain text from Markdown, `htmd` for HTML→Markdown, and `objc2-notification-center` for notifications.
- Clipboard encoding is assumed to be UTF-8.

## Contribution notes

- Preserve the clipboard-only contract (no stdio data paths).
- Clear the clipboard before writing converted output to avoid mixed formats.
- `to-html` must write both HTML and plain text (the Markdown stripped via `strip_markdown`) to the clipboard so consumers can pick either flavor.
- When adding tests, prefer mocked clipboard interactions.
