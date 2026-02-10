use std::{
    f64::consts::{E, PI, TAU},
    path::Path,
};

use chrono::Duration;
use hound::{WavSpec, WavWriter};
use lazy_static::lazy_static;

use rayon::prelude::*;
use std::collections::HashMap;

use pest::pratt_parser::{Assoc, Op, PrattParser};

use crate::{context::Context, expression::Expression, file::Musath};

pub mod body;
pub mod context;
pub mod expression;
pub mod file;
pub mod function;
pub mod header;

#[derive(pest_derive::Parser)]
#[grammar = "musath.pest"]
pub struct MusathParser;

type BuiltInFunction = dyn Fn(&Context, &[Expression]) -> f64 + Send + Sync + 'static;

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
        builtin_functions.insert(String::from("abs"), &|context, args| {
            args[0].eval(context).abs()
        });
        builtin_functions.insert(String::from("min"), &|context, args| {
            args[0].eval(context).min(args[1].eval(context))
        });
        builtin_functions.insert(String::from("max"), &|context, args| {
            args[0].eval(context).max(args[1].eval(context))
        });
        builtin_functions.insert(String::from("sin"), &|context, args| {
            args[0].eval(context).sin()
        });
        builtin_functions.insert(String::from("cos"), &|context, args| {
            args[0].eval(context).cos()
        });
        builtin_functions.insert(String::from("floor"), &|context, args| {
            args[0].eval(context).floor()
        });
        builtin_functions.insert(String::from("ceil"), &|context, args| {
            args[0].eval(context).ceil()
        });
        builtin_functions.insert(String::from("mix"), &|context, args| {
            args.iter().map(|arg| arg.eval(context)).sum::<f64>() / args.len() as f64
        });
        builtin_functions.insert(String::from("sum"), &|context, args| {
            if let Expression::Identifier(identifier) = &args[0] {
                let start = args[1].eval(context) as isize;
                let end = args[2].eval(context) as isize;
                let expression = &args[3];

                (start..end)
                    .map(move |value| {
                        let mut context = context.clone();
                        context.push_value(identifier.clone(), value as f64);
                        let result = expression.eval(&context);
                        result
                    })
                    .sum()
            } else {
                0.0
            }
        });
        builtin_functions.insert(String::from("prod"), &|context, args| {
            if let Expression::Identifier(identifier) = &args[0] {
                let start = args[1].eval(context) as isize;
                let end = args[2].eval(context) as isize;
                let expression = &args[3];

                (start..end)
                    .map(move |value| {
                        let mut context = context.clone();
                        context.push_value(identifier.clone(), value as f64);
                        let result = expression.eval(&context);
                        result
                    })
                    .product()
            } else {
                1.0
            }
        });
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

    (0..total_samples)
        .into_par_iter()
        .map(|i| {
            let t = i as f64 / spec.sample_rate as f64;

            musath.eval(t) as f32
        })
        .collect_into_vec(&mut mix);

    let mix_max = mix
        .iter()
        .copied()
        .filter(|mix| mix.is_normal() || *mix == 0.0)
        .max_by(|l, r| l.partial_cmp(r).unwrap())
        .unwrap();
    let mix_min = mix
        .iter()
        .map(|mix| mix.abs())
        .filter(|mix| mix.is_normal() || *mix == 0.0)
        .min_by(|l, r| l.partial_cmp(r).unwrap())
        .unwrap();
    let mix_abs_max = mix_max.max(mix_min);

    // let mix = mix
    //     .iter()
    //     .map(|mix| mix / mix_abs_max)
    //     .collect::<Vec<f32>>();
    println!("{:?}", mix);

    let mut writer = WavWriter::create(filename, spec)?;

    for sample in mix {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}
