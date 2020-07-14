use super::Reverb;
use crate::filter::{Allpass, FeedbackComb, Filter};

// https://ccrma.stanford.edu/software/stk/
// https://github.com/thestk/stk/blob/master/include/PRCRev.h
pub struct PRCRev {
    allpasses: [Allpass; 2],
    combs: [FeedbackComb; 2],
}

impl PRCRev {
    pub fn new(sample_rate: u32, t60: f64) -> Self {
        let scale_delay = |d| (f64::from(sample_rate) / 44100.0 * f64::from(d)) as usize;
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, scale_delay($delay)),
            )*]}
        }
        macro_rules! combs_from_delays {
            ($($delay:expr),*) => {[$(
                FeedbackComb::new(-(10.0 as f64).powf(-3.0 * f64::from($delay) / (44100.0 * t60)), scale_delay($delay)),
            )*]}
        }
        Self {
            allpasses: allpasses_from_delays![341, 613],
            combs: combs_from_delays![1557, 2137],
        }
    }
}

impl Reverb for PRCRev {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64) {
        let input = (x.0 + x.1) / 2.0;

        let allpass_output = self
            .allpasses
            .iter_mut()
            .fold(input, |output, a| a.process_sample(output));

        let comb_output: Vec<_> = self
            .combs
            .iter_mut()
            .map(|c| 0.5 * (input + c.process_sample(allpass_output)))
            .collect();

        (comb_output[0], comb_output[1])
    }
}
