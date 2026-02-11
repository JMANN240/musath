pub mod body;
pub mod composition;
pub mod context;
pub mod document;
pub mod expression;
pub mod function;
pub mod header;
pub mod renderer;
pub mod wave_provider;

#[derive(pest_derive::Parser)]
#[grammar = "musath.pest"]
pub struct MusathParser;
