use std::f64::consts::TAU;

use musath::{
    composition::Composition,
    renderer::{Renderer, parallel_renderer::ParallelRenderer},
};

fn main() {
    ParallelRenderer::default()
        .render(&Composition::from_function("beat", 60.0, output))
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

fn base_freq(t: f64) -> f64 {
    440.0 * note_ratio().powf((t / 4.0).floor().rem_euclid(4.0))
}

fn voice(t: f64) -> f64 {
    step(
        t,
        base_freq(t),
    ) / 10.0
        * (1..4).map(|n| -step(t, n as f64) + 1.0).product::<f64>()
}

fn square(t: f64) -> f64 {
    (1..5)
        .map(|n| voice(t - (n as f64 * 0.25 - 0.25)) / 4.0f64.powf(n as f64 - 1.0))
        .sum()
}

fn saw(t: f64) -> f64 {
    let freq = 4.0/base_freq(t);

    2.0 * t.rem_euclid(freq) / freq - 1.0
}

fn note_ratio() -> f64 {
    2.0f64.powf(1.0 / 12.0)
}

fn output(t: f64) -> f64 {
    noise(t) * decay(t, 1.0, 8.0, 0.5)
        + (110.0 * t * TAU).sin() * decay(t, 1.0, 8.0, 0.0)
        + square(t)
        + saw(t) / 16.0 * (((4.0 * t * TAU).sin() + 1.0) / 2.0).powf(2.0)
}
