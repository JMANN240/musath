use crate::{document::Document, wave_provider::WaveProvider};

pub struct Composition {
    title: Option<String>,
    duration: Option<f64>,
    wave_provider: Box<dyn WaveProvider + Send + Sync>,
}

impl Composition {
    pub fn from_document(document: Document) -> Self {
        Self {
            title: document.header().title().map(ToString::to_string),
            duration: document.header().duration(),
            wave_provider: Box::new(document),
        }
    }

    pub fn from_function<F: Fn(f64) -> f64 + Send + Sync + 'static>(
        title: impl Into<String>,
        duration: f64,
        function: F,
    ) -> Self {
        Self {
            title: Some(title.into()),
            duration: Some(duration),
            wave_provider: Box::new(function) as Box<dyn WaveProvider + Send + Sync>,
        }
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn duration(&self) -> Option<f64> {
        self.duration
    }

    pub fn wave_provider(&self) -> &dyn WaveProvider {
        self.wave_provider.as_ref()
    }

}
