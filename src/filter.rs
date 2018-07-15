use delay::Delay;

pub trait Filter {
    fn process_sample(&mut self, x: f64) -> f64;
}

// https://ccrma.stanford.edu/~jos/pasp/Allpass_Two_Combs.html
pub struct Allpass {
    am: f64,
    b0: f64,
    zm: Delay,
}

impl Allpass {
    pub fn new(am: f64, b0: f64, m: usize) -> Self {
        Self {
            am,
            b0,
            zm: Delay::new(m),
        }
    }
}

impl Filter for Allpass {
    fn process_sample(&mut self, x: f64) -> f64 {
        let v = x - self.am * self.zm.output();
        let output = self.b0 * v + self.zm.output();
        self.zm.input(v);
        output
    }
}

// https://ccrma.stanford.edu/~jos/pasp/Feedback_Comb_Filters.html
pub struct FeedbackComb {
    am: f64,
    zm: Delay,
}

impl FeedbackComb {
    pub fn new(am: f64, m: usize) -> Self {
        Self {
            am,
            zm: Delay::new(m),
        }
    }
}

impl Filter for FeedbackComb {
    fn process_sample(&mut self, x: f64) -> f64 {
        let v = x - self.am * self.zm.output();
        self.zm.input(v);
        v
    }
}
