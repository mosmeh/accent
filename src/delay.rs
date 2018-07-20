use std::collections::VecDeque;

pub struct Delay {
    buffer: VecDeque<f64>,
}

impl Delay {
    pub fn new(length: usize) -> Self {
        let mut buffer = VecDeque::with_capacity(length);
        for _ in 0..length {
            buffer.push_back(0.0);
        }

        Self { buffer }
    }

    pub fn input(&mut self, x: f64) {
        self.buffer.pop_front();
        self.buffer.push_back(x);
    }

    pub fn output(&self) -> f64 {
        *self.buffer.front().unwrap()
    }
}
