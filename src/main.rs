use std::path::PathBuf;

use musath::{
    MusathParser, Rule, composition::Composition, document::Document, renderer::{Renderer, parallel_renderer::ParallelRenderer, serial_renderer::SerialRenderer}
};
use pest::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(clap::Parser)]
struct Args {
    path: PathBuf,

    #[arg(short, long, value_enum, default_value_t = RendererOption::Parallel)]
    renderer: RendererOption,
}

#[derive(Clone, clap::ValueEnum)]
enum RendererOption {
    Serial,
    Parallel,
}

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let args = <Args as clap::Parser>::parse();

    let unparsed_file = std::fs::read_to_string(args.path).expect("cannot read file");

    info!("Parsing...");
    let document =
        Document::parse(&mut MusathParser::parse(Rule::document, &unparsed_file).unwrap());
    info!("Parsed!");

    info!("Rendering...");
    let renderer = match args.renderer {
        RendererOption::Serial => Box::new(SerialRenderer::default()) as Box<dyn Renderer>,
        RendererOption::Parallel => Box::new(ParallelRenderer::default()) as Box<dyn Renderer>,
    };

    renderer.render(&Composition::from_document(document)).unwrap();

    info!("Rendered!");
}
