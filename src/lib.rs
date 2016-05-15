extern crate rand;

use rand::Rng;
use std::fmt;

pub struct Karplus {
    frequency: f32,
    sample_rate: u32,
    buffer: Vec<f32>
}

impl Karplus {
    pub fn new(frequency: f32, sample_rate: u32) -> Karplus {
        let size = (sample_rate as f32 / frequency) as usize;
        let mut v: Vec<f32> = Vec::with_capacity(size);
        let mut r = rand::thread_rng();
        for _ in 0..size {
            v.push((r.gen::<f32>() * 2.0) -1.0);
        }
        Karplus{
            frequency: frequency,
            sample_rate: sample_rate,
            buffer: v
        }
    }

    pub fn sample(&mut self, damping: f32) -> f32 {
        let v: f32 = self.buffer.remove(0);
        let s: f32 = (v + self.buffer[0])*0.5 * damping;

        self.buffer.push(s);
        s
    }
}

impl fmt::Display for Karplus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.frequency, self.sample_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karplus_new() {
        let k = Karplus::new(440.0, 4400);
        assert_eq!(k.frequency, 440.0);
        assert_eq!(k.sample_rate, 4400);
        assert_eq!(k.buffer.len(), 10);
    }

    #[test]
    fn test_karplus_sample() {
        let mut k = Karplus::new(440.0, 44100);
        k.buffer = vec![1.0, 0.0];
        let sample = k.sample(1.0);
        assert_eq!(sample, 0.5)
    }
}
