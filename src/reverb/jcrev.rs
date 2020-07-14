use super::Reverb;
use crate::filter::{Allpass, FeedbackComb, Filter};

// https://ccrma.stanford.edu/~jos/pasp/Schroeder_Reverberators.html
pub struct JCRev {
    allpasses: [Allpass; 3],
    combs: [FeedbackComb; 4],
}

impl JCRev {
    pub fn new(sample_rate: u32) -> Self {
        let scale_delay = |d| (f64::from(sample_rate) / 25000.0 * f64::from(d)) as usize;
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, scale_delay($delay)),
            )*]}
        }
        macro_rules! combs_from_feedbacks_and_delays {
            ($($am:expr, $delay:expr);*) => {[$(
                FeedbackComb::new($am, scale_delay($delay)),
            )*]}
        }
        Self {
            allpasses: allpasses_from_delays![347, 113, 37],
            combs: combs_from_feedbacks_and_delays![-0.773, 1687; -0.802, 1601; -0.753, 2053; -0.733, 2251],
        }
    }
}

impl Reverb for JCRev {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64) {
        let input = (x.0 + x.1) / 2.0;

        let allpass_output = self
            .allpasses
            .iter_mut()
            .fold(input, |output, a| a.process_sample(output));

        let comb_output: Vec<_> = self
            .combs
            .iter_mut()
            .map(|c| c.process_sample(allpass_output))
            .collect();

        (
            comb_output[0] + comb_output[2],
            comb_output[1] + comb_output[3],
        )
    }
}
