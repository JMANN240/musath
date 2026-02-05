use std::{f64::consts::{E, PI, TAU}, path::Path, thread::sleep};

use chrono::Duration;
use hound::{WavSpec, WavWriter};
use lazy_static::lazy_static;

use std::collections::HashMap;

use pest::{pratt_parser::{Assoc, Op, PrattParser}};

use crate::file::Musath;

pub mod expression;
pub mod file;
pub mod function;

#[derive(pest_derive::Parser)]
#[grammar = "musath.pest"]
pub struct MusathParser;

type BuiltInFunction = dyn Fn(&[f64]) -> f64 + Send + Sync + 'static;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        PrattParser::new()
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
            .op(Op::infix(Rule::modulo, Assoc::Left))
            .op(Op::infix(Rule::pow, Assoc::Right))
            .op(Op::prefix(Rule::neg))
            .op(Op::postfix(Rule::fac))
    };

    static ref BUILT_IN_FUNCTIONS: HashMap<String, &'static BuiltInFunction> = {
        let mut builtin_functions = HashMap::<String, &'static BuiltInFunction>::new();
        builtin_functions.insert(String::from("sin"), &|args: &[f64]| args[0].sin());
        builtin_functions.insert(String::from("cos"), &|args: &[f64]| args[0].cos());
        builtin_functions.insert(String::from("mix"), &|args: &[f64]| args.iter().sum::<f64>() / args.len() as f64);
        builtin_functions
    };

    static ref BUILT_IN_NUMBERS: HashMap<String, f64> = {
        let mut builtin_numbers = HashMap::new();
        builtin_numbers.insert(String::from("pi"), PI);
        builtin_numbers.insert(String::from("tau"), TAU);
        builtin_numbers.insert(String::from("e"), E);
        builtin_numbers
    };
}

pub fn render<P: AsRef<Path>>(
    filename: P,
    duration: Duration,
    musath: Musath,
) -> Result<(), hound::Error> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let total_samples = (spec.channels as f64 * duration.as_seconds_f64() * spec.sample_rate as f64)
        .ceil() as usize;

    let mut mix = vec![0.0f32; total_samples];

    for i in 0..total_samples {
        let t = i as f64 / spec.sample_rate as f64;

        mix[i] = musath.eval(t) as f32;
    }

    let mut writer = WavWriter::create(filename, spec)?;

    for sample in mix {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}
