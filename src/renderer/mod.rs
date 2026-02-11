use crate::composition::Composition;

pub mod parallel_renderer;
pub mod serial_renderer;

pub trait Renderer {
    fn render(&self, composition: &Composition) -> Result<(), hound::Error>;
}
