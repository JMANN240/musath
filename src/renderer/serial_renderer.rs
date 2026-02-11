use hound::{WavSpec, WavWriter};
use tracing::debug;

use crate::{composition::Composition, document::Document, renderer::Renderer};

pub struct SerialRenderer {
    spec: WavSpec,
}

impl SerialRenderer {
    pub fn new(spec: WavSpec) -> Self {
        Self { spec }
    }
}

impl Default for SerialRenderer {
    fn default() -> Self {
        Self::new(WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        })
    }
}

impl Renderer for SerialRenderer {
    fn render(&self, composition: &Composition) -> Result<(), hound::Error> {
        debug!("creating spec");

        let title = composition.title().map(String::as_str).unwrap_or("output");

        let duration_seconds = composition.duration().unwrap_or(10.0);

        debug!("calculating total samples");
        let total_samples =
            (self.spec.channels as f64 * duration_seconds * self.spec.sample_rate as f64).ceil()
                as usize;

        debug!("allocating samples vector");
        let mut mix = vec![0.0f32; total_samples];

        debug!("rendering");
        for i in 0..total_samples {
            let t = i as f64 / self.spec.sample_rate as f64;

            let value = composition.wave_provider().value_at_time(t) as f32;

            debug!("{}/{}", i, total_samples);

            mix[i] = value;
        }

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
