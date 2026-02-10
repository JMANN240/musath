use std::path::PathBuf;

use chrono::Duration;
use musath::{file::Musath, header::HeaderValue, render};

#[derive(clap::Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let unparsed_file = std::fs::read_to_string(args.path).expect("cannot read file");

    let file = Musath::parse(&unparsed_file);

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
}
