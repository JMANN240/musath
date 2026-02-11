use crate::document::Document;

pub mod parallel_renderer;
pub mod serial_renderer;

pub trait Renderer {
    fn render(&self, document: Document) -> Result<(), hound::Error>;
}
