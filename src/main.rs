use clap::{Parser, Subcommand};
use clipboard_rs::{Clipboard, ClipboardContent, ClipboardContext, ContentFormat};
use markdown::to_html as md_to_html;
use objc2::rc::Retained;
use objc2_foundation::{NSString, NSUserNotification, NSUserNotificationCenter};
use strip_markdown::strip_markdown;
use thiserror::Error;

/// Command line entry point for marklip.
#[derive(Parser, Debug)]
#[command(name = "marklip", version, disable_help_subcommand = true)]
struct Cli {
    /// Suppress stderr output.
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Send messages via Notification Center instead of stderr.
    #[arg(short, long, global = true)]
    notify: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Convert clipboard Markdown text to HTML and write back as HTML only.
    ToHtml,
    /// Convert clipboard HTML to Markdown text and write back as plain text only.
    ToMd,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("required clipboard content is missing")]
    MissingClipboard,
    #[error("conversion failed")]
    ConversionFailed,
    #[error("clipboard operation failed: {0}")]
    Clipboard(String),
}

fn main() {
    let cli = Cli::parse();

    let mut ctx = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(err) => {
            emit_message("Failed to access clipboard context", &cli, true);
            eprintln!("{err}");
            std::process::exit(99);
        }
    };

    let result = match cli.command {
        Command::ToHtml => convert_to_html(&mut ctx),
        Command::ToMd => convert_to_md(&mut ctx),
    };

    match result {
        Ok(msg) => {
            emit_message(&msg, &cli, false);
            std::process::exit(0);
        }
        Err(err) => {
            handle_error(err, &cli);
        }
    }
}

fn convert_to_html(ctx: &mut ClipboardContext) -> Result<String, AppError> {
    if !ctx.has(ContentFormat::Text) {
        return Err(AppError::MissingClipboard);
    }

    let input = ctx
        .get_text()
        .map_err(|e| AppError::Clipboard(e.to_string()))?;

    let html = md_to_html(&input);
    let plain_text = strip_markdown(&input);

    ctx.clear()
        .map_err(|e| AppError::Clipboard(e.to_string()))?;
    ctx.set(vec![
        ClipboardContent::Html(html),
        ClipboardContent::Text(plain_text),
    ])
    .map_err(|e| AppError::Clipboard(e.to_string()))?;

    Ok("Converted Markdown to HTML and copied to clipboard.".to_string())
}

fn convert_to_md(ctx: &mut ClipboardContext) -> Result<String, AppError> {
    if !ctx.has(ContentFormat::Html) {
        return Err(AppError::MissingClipboard);
    }

    let html = ctx
        .get_html()
        .map_err(|e| AppError::Clipboard(e.to_string()))?;

    let markdown = htmd::convert(&html).map_err(|_| AppError::ConversionFailed)?;

    ctx.clear()
        .map_err(|e| AppError::Clipboard(e.to_string()))?;
    ctx.set_text(markdown)
        .map_err(|e| AppError::Clipboard(e.to_string()))?;

    Ok("Converted HTML to Markdown and copied to clipboard.".to_string())
}

fn handle_error(err: AppError, cli: &Cli) -> ! {
    let (code, message) = match &err {
        AppError::MissingClipboard => (1, "Required clipboard format is missing."),
        AppError::ConversionFailed => (2, "Conversion failed."),
        AppError::Clipboard(msg) => (255, &**msg),
    };

    emit_message(message, cli, true);
    std::process::exit(code);
}

fn emit_message(message: &str, cli: &Cli, is_error: bool) {
    if cli.notify {
        if let Err(err) = send_notification("marklip", message) {
            eprintln!("{message}");
            eprintln!("{err}");
        }
    } else if !cli.quiet {
        eprintln!("{message}");
    } else if cli.quiet && is_error {
        // Quiet suppresses output unless notify is used; errors stay silent here.
    }
}

fn send_notification(title: &str, body: &str) -> Result<(), AppError> {
    // NSUserNotification is deprecated but sufficient for simple local notifications.
    let notification: Retained<NSUserNotification> = NSUserNotification::new();
    let title_ns = NSString::from_str(title);
    let body_ns = NSString::from_str(body);
    notification.setTitle(Some(&title_ns));
    notification.setInformativeText(Some(&body_ns));

    let center = NSUserNotificationCenter::defaultUserNotificationCenter();
    center.deliverNotification(&notification);
    Ok(())
}
