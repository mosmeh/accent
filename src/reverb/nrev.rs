use super::Reverb;
use crate::filter::{Allpass, FeedbackComb, FeedforwardComb, Filter};

// https://ccrma.stanford.edu/software/stk/
// https://github.com/thestk/stk/blob/master/include/NRev.h
pub struct NRev {
    fb_combs: [FeedbackComb; 6],
    ff_comb: FeedforwardComb,
    allpasses: [Allpass; 6],
}

impl NRev {
    pub fn new(sample_rate: u32, t60: f64) -> Self {
        let scale_delay = |d| (f64::from(sample_rate) / 25641.0 * f64::from(d)) as usize;
        macro_rules! combs_from_delays {
            ($($delay:expr),*) => {[$(
                FeedbackComb::new(-(10.0 as f64).powf(-3.0 * f64::from($delay) / (44100.0 * t60)), scale_delay($delay)),
            )*]}
        }
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, scale_delay($delay)),
            )*]}
        }
        Self {
            fb_combs: combs_from_delays![1433, 1601, 1867, 2053, 2251, 2399],
            ff_comb: FeedforwardComb::new(0.3, 0.7, 1),
            allpasses: allpasses_from_delays![347, 113, 37, 59, 53, 43],
        }
    }
}

impl Reverb for NRev {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64) {
        let input = (x.0 + x.1) / 2.0;

        let comb_output = self
            .fb_combs
            .iter_mut()
            .map(|c| c.process_sample(input))
            .sum();

        let allpass_output = self.allpasses[0..3]
            .iter_mut()
            .fold(comb_output, |output, a| a.process_sample(output));

        let lowpass_output =
            self.allpasses[3].process_sample(self.ff_comb.process_sample(allpass_output));

        let output: Vec<_> = self.allpasses[4..6]
            .iter_mut()
            .map(|a| 0.3 * a.process_sample(lowpass_output) + 0.7 * input)
            .collect();

        (output[0], output[1])
    }
}
