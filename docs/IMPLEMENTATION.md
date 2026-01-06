# Marklip Implementation Notes

## Architecture
- Entry point parses subcommands (`to-html`, `to-md`) and global flags (`--help`, `--version`, `--quiet`, `--notify`).
- No stdin/stdout I/O; all data flow goes through the macOS clipboard.
- Unified error type maps operational failures to exit codes: 1 (missing required clipboard format), 2 (conversion failure), 255 (other errors).

## Clipboard handling
- Use `clipboard-rs` to read/write macOS clipboard types.
- `to-html`: fetch plain text (`NSPasteboard.PasteboardType.string`); error if absent. After conversion, clear clipboard and write both HTML and plain text. The plain text is produced by stripping Markdown formatting (via `strip_markdown`) from the original input to keep a clean text representation alongside the HTML.
- `to-md`: fetch HTML (`NSPasteboard.PasteboardType.html`); ignore plain text. Error if HTML absent. After conversion, clear clipboard and write plain text only.
- Assume UTF-8 for encoding and decoding operations.

## Conversion pipelines
- Markdown → HTML: use the `markdown` crate; minimal configuration expected, preserving UTF-8. Additionally derive a plain-text rendition with `strip_markdown` to populate the clipboard text flavor when running `to-html`.
- HTML → Markdown: use the `htmd` crate to down-convert HTML nodes to Markdown text.

## Notifications and logging
- Standard path: emit messages to stderr unless `--quiet` is present.
- Notification path: when `--notify` is set, use `objc2-notification-center` to deliver notifications instead of stderr. On notification failure, log to stderr even if `--quiet` is active; success still exits 0.
- Success messages are always produced (stderr or notification) unless suppressed by `--quiet` without `--notify`.

## Exit code mapping
- Missing required clipboard data: 1.
- Conversion errors (parser/render failures): 2.
- Any other runtime or unexpected error: 255.

## Operational notes
- Clear the clipboard before writing the converted content to avoid mixed formats.
- Keep user-facing strings in English.
- Target Apple Silicon macOS builds only; no cross-platform shims needed.

## Future work
- Add automated tests with mocked clipboard interactions.
- Consider a `--stdout` debugging flag if development workflow needs it (not in current spec).
