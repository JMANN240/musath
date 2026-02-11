use std::f64::consts::TAU;

use musath::{
    composition::Composition,
    renderer::{Renderer, parallel_renderer::ParallelRenderer},
};

fn main() {
    ParallelRenderer::default()
        .render(&Composition::from_function("beat", 10.0, output))
        .unwrap();
}

fn decay(t: f64, period: f64, tension: f64, phase_shift: f64) -> f64 {
    (1.0 - ((t - phase_shift).rem_euclid(period)) / period).powf(tension)
}

fn noise(t: f64) -> f64 {
    (2.0f64.powf(t + 100.0)).sin().powf(2.0)
}

fn step(t: f64, freq: f64) -> f64 {
    ((t * freq * 2.0).rem_euclid(2.0)).floor()
}

fn voice(t: f64) -> f64 {
    step(t, 440.0) / 10.0 * (1..4).map(|n| -step(t, n as f64) + 1.0).product::<f64>()
}

fn square(t: f64) -> f64 {
    (1..5).map(|n| voice(t - (n as f64 * 0.25 - 0.25)) / 4.0f64.powf(n as f64 - 1.0)).sum()
}

fn output(t: f64) -> f64 {
    noise(t) * decay(t, 1.0, 8.0, 0.5) +
    (110.0 * t * TAU).sin() * decay(t, 1.0, 8.0, 0.0) +
    square(t)
}
