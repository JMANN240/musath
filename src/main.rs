use std::path::PathBuf;

use chrono::Duration;
use musath::{MusathParser, Rule, file::Musath, render};
use pest::Parser;

#[derive(clap::Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let unparsed_file = std::fs::read_to_string(args.path).expect("cannot read file");

    let pairs = MusathParser::parse(Rule::file, &unparsed_file).expect("could not parse file");

    let file = Musath::parse(pairs);

    render("output.wav", Duration::new(1, 0).unwrap(), file).unwrap();
}
