pub trait WaveProvider {
    fn value_at_time(&self, t: f64) -> f64;
}

impl <F: Fn(f64) -> f64> WaveProvider for F {
    fn value_at_time(&self, t: f64) -> f64 {
        self(t)
    }
}
