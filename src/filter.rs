use crate::delay::Delay;

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

pub struct FeedforwardComb {
    b0: f64,
    bm: f64,
    zm: Delay,
}

impl FeedforwardComb {
    pub fn new(b0: f64, bm: f64, m: usize) -> Self {
        Self {
            b0,
            bm,
            zm: Delay::new(m),
        }
    }
}

impl Filter for FeedforwardComb {
    fn process_sample(&mut self, x: f64) -> f64 {
        let output = self.b0 * x + self.bm * self.zm.output();
        self.zm.input(x);
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

// https://ccrma.stanford.edu/~jos/pasp/Lowpass_Feedback_Comb_Filter.html
pub struct LowpassFeedbackComb {
    f: f64,
    d: f64,
    z1: Delay,
    zn: Delay,
}

impl LowpassFeedbackComb {
    pub fn new(f: f64, d: f64, n: usize) -> Self {
        Self {
            f,
            d,
            z1: Delay::new(1),
            zn: Delay::new(n),
        }
    }
}

impl Filter for LowpassFeedbackComb {
    fn process_sample(&mut self, x: f64) -> f64 {
        let filterstore = (1.0 - self.d) * self.zn.output() + self.d * self.z1.output();
        let y = x + self.f * filterstore;
        self.z1.input(filterstore);
        self.zn.input(y);
        y
    }
}
