use super::Reverb;
use crate::filter::{Allpass, FeedbackComb, Filter};

// https://ccrma.stanford.edu/~jos/pasp/Example_Schroeder_Reverberators.html
pub struct SATREV {
    combs: [FeedbackComb; 4],
    allpasses: [Allpass; 3],
}

impl SATREV {
    pub fn new(sample_rate: u32) -> Self {
        let scale_delay = |d| (f64::from(sample_rate) / 25000.0 * f64::from(d)) as usize;
        macro_rules! combs_from_feedbacks_and_delays {
            ($($am:expr, $delay:expr);*) => {[$(
                FeedbackComb::new($am, scale_delay($delay)),
            )*]}
        }
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, scale_delay($delay)),
            )*]}
        }
        Self {
            combs: combs_from_feedbacks_and_delays![-0.805, 901; -0.827, 778; -0.783, 1011; -0.764, 1123],
            allpasses: allpasses_from_delays![125, 42, 12],
        }
    }
}

impl Reverb for SATREV {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64) {
        let input = (x.0 + x.1) / 2.0;

        let comb_output = self.combs.iter_mut().map(|c| c.process_sample(input)).sum();

        let allpass_output = self
            .allpasses
            .iter_mut()
            .fold(comb_output, |output, a| a.process_sample(output));

        (allpass_output, -allpass_output)
    }
}
