use std::path::PathBuf;

use chrono::Duration;
use musath::{MusathParser, Rule, document::Document, header::HeaderValue, render};
use pest::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(clap::Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let args = <Args as clap::Parser>::parse();

    let unparsed_file = std::fs::read_to_string(args.path).expect("cannot read file");

    info!("Parsing...");
    let file = Document::parse(&mut MusathParser::parse(Rule::document, &unparsed_file).unwrap());
    info!("Parsed!");

    let output_filename_header_value = file
        .header()
        .key_values()
        .get("TITLE")
        .cloned()
        .unwrap_or(HeaderValue::String(String::from("output")));

    let duration_header_value = file
        .header()
        .key_values()
        .get("DURATION")
        .cloned()
        .unwrap_or(HeaderValue::Number(30.0));

    info!("Rendering...");
    if let HeaderValue::String(output_filename) = output_filename_header_value
        && let HeaderValue::Number(duration) = duration_header_value
    {
        render(
            format!("{}.wav", output_filename),
            Duration::new(duration as i64, 0).unwrap(),
            file,
        )
        .unwrap();
    }
    info!("Rendered!");
}
