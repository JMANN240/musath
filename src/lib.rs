use std::{path::Path, sync::{Arc, Mutex}};

use chrono::Duration;
use hound::{WavSpec, WavWriter};

use rayon::prelude::*;
use tracing::debug;

use crate::document::Document;

pub mod body;
pub mod context;
pub mod document;
pub mod expression;
pub mod function;
pub mod header;
pub mod renderer;

#[derive(pest_derive::Parser)]
#[grammar = "musath.pest"]
pub struct MusathParser;

pub fn render<P: AsRef<Path>>(
    filename: P,
    duration: Duration,
    document: Document,
) -> Result<(), hound::Error> {
    debug!("creating spec");
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    debug!("calculating total samples");
    let total_samples = (spec.channels as f64 * duration.as_seconds_f64() * spec.sample_rate as f64)
        .ceil() as usize;

    debug!("allocating samples vector");
    let mut mix = vec![0.0f32; total_samples];

    let samples_completed = Arc::new(Mutex::new(0));

    debug!("rendering");
    (0..total_samples)
        .into_par_iter()
        .map(|i| {
            let t = i as f64 / spec.sample_rate as f64;

            let value = document.eval(t) as f32;

            let mut lock = samples_completed.lock().unwrap();
            *lock += 1;
            debug!("{}/{}", *lock, total_samples);

            value
        })
        .collect_into_vec(&mut mix);

    debug!("creating writer");
    let mut writer = WavWriter::create(filename, spec)?;

    debug!("writing samples");
    for sample in mix {
        writer.write_sample(sample)?;
    }

    debug!("finalizing writer");
    writer.finalize()?;

    Ok(())
}
