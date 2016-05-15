extern crate rand;

use rand::Rng;

struct Karplus {
    buffer: Vec<f64>
}

impl Karplus {
    fn new(frequency: f64, sampling_rate: u64) -> Karplus {
        let size = (sampling_rate as f64 / frequency) as usize;
        let mut v: Vec<f64> = Vec::with_capacity(size);
        let mut r = rand::thread_rng();
        for _ in 0..size {
            v.push(r.gen());
        }
        Karplus{
            buffer: v
        }
    }

    fn sample(&mut self, damping: f64) -> f64 {
        let v: f64 = self.buffer.remove(0);
        let s: f64 = (v + self.buffer[0])*0.5 * damping;

        self.buffer.push(s);
        s
    }
}
