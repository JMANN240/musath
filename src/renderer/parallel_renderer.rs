use std::sync::{Arc, Mutex};

use hound::{WavSpec, WavWriter};
use rayon::prelude::*;
use tracing::debug;

use crate::{document::Document, renderer::Renderer};



pub struct ParallelRenderer {
    spec: WavSpec,
}

impl ParallelRenderer {
    pub fn new(spec: WavSpec) -> Self {
        Self { spec }
    }
}

impl Default for ParallelRenderer {
    fn default() -> Self {
        Self::new(WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        })
    }
}

impl Renderer for ParallelRenderer {
    fn render(&self, document: Document) -> Result<(), hound::Error> {
        debug!("creating spec");

        let title = document.header().title().unwrap_or("output");

        let duration_seconds = document.header().duration().unwrap_or(10.0);

        debug!("calculating total samples");
        let total_samples =
            (self.spec.channels as f64 * duration_seconds * self.spec.sample_rate as f64).ceil()
                as usize;

        debug!("allocating samples vector");
        let mut mix = vec![0.0f32; total_samples];

        let samples_completed = Arc::new(Mutex::new(0));

        debug!("rendering");
        (0..total_samples)
            .into_par_iter()
            .map(|i| {
                let t = i as f64 / self.spec.sample_rate as f64;

                let value = document.eval(t) as f32;

                let mut lock = samples_completed.lock().unwrap();
                *lock += 1;
                debug!("{}/{}", *lock, total_samples);

                value
            })
            .collect_into_vec(&mut mix);

        debug!("creating writer");
        let mut writer = WavWriter::create(format!("{}.wav", title), self.spec)?;

        debug!("writing samples");
        for sample in mix {
            writer.write_sample(sample)?;
        }

        debug!("finalizing writer");
        writer.finalize()?;

        Ok(())
    }
}