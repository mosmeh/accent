use delay::Delay;

pub struct Allpass {
    a: f64,
    b: f64,
    delay: Delay,
}

impl Allpass {
    pub fn new(a: f64, b: f64, delay: usize) -> Self {
        Self {
            a,
            b,
            delay: Delay::new(delay),
        }
    }

    pub fn process_sample(&mut self, x: f64) -> f64 {
        let v = x - self.a * self.delay.output();
        let output = self.b * v + self.delay.output();
        self.delay.input(v);
        output
    }
}

pub struct FeedbackComb {
    a: f64,
    delay: Delay,
}

impl FeedbackComb {
    pub fn new(a: f64, delay: usize) -> Self {
        Self {
            a,
            delay: Delay::new(delay),
        }
    }

    pub fn process_sample(&mut self, x: f64) -> f64 {
        let v = x - self.a * self.delay.output();
        self.delay.input(v);
        v
    }
}
