use clap::Parser;
use lib::{CliAmberVersion, detect_amber_version};
use std::env::temp_dir;
use tracing::subscriber;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Version of the Amber language to use.
    #[arg(value_enum, long, short, default_value = "auto")]
    amber_version: CliAmberVersion,
}

fn main() {
    let cache_dir = temp_dir().join("amber-lsp");
    let file_appender = tracing_appender::rolling::hourly(cache_dir, "amber-lsp.log");
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Don't display the thread ID an event was recorded on
        .with_thread_ids(false)
        // Don't display the event's target (module path)
        .with_target(false)
        // Log when entering and exiting spans
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        // log to a file
        .with_writer(non_blocking_writer)
        // Disabled ANSI color codes for better compatibility with some terminals
        .with_ansi(false)
        // Build the subscriber
        .finish();

    // use that subscriber to process traces emitted after this point
    subscriber::set_global_default(subscriber).expect("Could not set global default subscriber");

    let args = Args::parse();

    let amber_version = if args.amber_version == CliAmberVersion::Auto {
        detect_amber_version()
    } else {
        args.amber_version.into()
    };
}
