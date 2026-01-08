# Marklip Implementation Notes

## Architecture

- Entry point parses subcommands (`to-html`, `to-md`) and global flags (`--help`, `--version`, `--quiet`, `--notify`).
- No stdin/stdout I/O; all data flow goes through the system clipboard.
- Unified error type maps operational failures to exit codes: 1 (missing required clipboard format), 2 (conversion failure), 255 (other errors).
- `auto` subcommand selects the conversion path at runtime: prefers HTML → Markdown; otherwise uses non-empty plain text → HTML; treats empty text or absent content as missing clipboard data.

## Clipboard handling

- Use `clipboard-rs` to read/write platform clipboard types.
- `to-html`: fetch plain text (string flavor); error if absent. After conversion, clear clipboard and write both HTML and plain text. The plain text is produced by stripping Markdown formatting (via `strip_markdown`) from the original input to keep a clean text representation alongside the HTML.
- `to-md`: fetch HTML; ignore plain text. Error if HTML absent. After conversion, clear clipboard and write plain text only.
- Assume UTF-8 for encoding and decoding operations.

## Conversion pipelines

- Markdown → HTML: use the `markdown` crate; minimal configuration expected, preserving UTF-8. Additionally derive a plain-text rendition with `strip_markdown` to populate the clipboard text flavor when running `to-html`.
- HTML → Markdown: use the `htmd` crate to down-convert HTML nodes to Markdown text.

## Notifications and logging

- Standard path: emit messages to stderr unless `--quiet` is present.
- Notification path: when `--notify` is set, use `notify-rust` (platform backends) to deliver notifications instead of stderr. On notification failure, log to stderr even if `--quiet` is active; success still exits 0.
- Success messages are always produced (stderr or notification) unless suppressed by `--quiet` without `--notify`.

## Exit code mapping

- Missing required clipboard data: 1.
- Conversion errors (parser/render failures): 2.
- Any other runtime or unexpected error: 255.

## Operational notes

- Clear the clipboard before writing the converted content to avoid mixed formats.
- Keep user-facing strings in English.
- Primary development is on macOS, but the code relies on `clipboard-rs` and `notify-rust` backends and should run anywhere those crates support.

## Future work

- Add automated tests with mocked clipboard interactions.
- Consider a `--stdout` debugging flag if development workflow needs it (not in current spec).

## Notification notes (notify-rust)

- Dependency: `notify-rust` (macOS uses `mac-notification-sys`; other platforms use their respective backends).
- Flow: on `--notify`, build a notification with summary/body and call `.show()`. Errors map to the generic exit bucket (255) and still log to stderr even if `--quiet` was requested.
- Bundle note: unlike direct `UNUserNotificationCenter`, `notify-rust` works from a bare CLI binary; no CFBundleIdentifier required on macOS. Other platforms follow their backend conventions.
- Testing: notifications remain hard to automate; prefer mocking the notifier interface. Real delivery stays a manual check.
