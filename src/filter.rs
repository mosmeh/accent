use delay::Delay;

pub trait Filter {
    fn process_sample(&mut self, x: f64) -> f64;
}

// https://ccrma.stanford.edu/~jos/pasp/Allpass_Two_Combs.html
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
}

impl Filter for Allpass {
    fn process_sample(&mut self, x: f64) -> f64 {
        let v = x - self.a * self.delay.output();
        let output = self.b * v + self.delay.output();
        self.delay.input(v);
        output
    }
}

// https://ccrma.stanford.edu/~jos/pasp/Feedback_Comb_Filters.html
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
}

impl Filter for FeedbackComb {
    fn process_sample(&mut self, x: f64) -> f64 {
        let v = x - self.a * self.delay.output();
        self.delay.input(v);
        v
    }
}
