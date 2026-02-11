use std::path::Path;

use chrono::Duration;
use hound::{WavSpec, WavWriter};

use rayon::prelude::*;

use crate::document::Document;

pub mod body;
pub mod context;
pub mod document;
pub mod expression;
pub mod function;
pub mod header;

#[derive(pest_derive::Parser)]
#[grammar = "musath.pest"]
pub struct MusathParser;

pub fn render<P: AsRef<Path>>(
    filename: P,
    duration: Duration,
    document: Document,
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

            document.eval(t) as f32
        })
        .collect_into_vec(&mut mix);

    let mut writer = WavWriter::create(filename, spec)?;

    for sample in mix {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}
