use super::Reverb;
use crate::delay::Delay;
use crate::filter::{Allpass, FeedforwardComb, Filter};

// https://ccrma.stanford.edu/software/stk/
// https://github.com/thestk/stk/blob/master/include/JCRev.h
pub struct STKJCRev {
    allpasses: [Allpass; 3],
    combs: [(Delay, FeedforwardComb); 4],
    out_delays: [Delay; 2],
}

impl STKJCRev {
    pub fn new(sample_rate: u32, t60: f64) -> Self {
        let scale_delay = |d| (f64::from(sample_rate) / 44100.0 * f64::from(d)) as usize;
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, scale_delay($delay)),
            )*]}
        }
        macro_rules! combs_from_delays {
            ($($delay:expr),*) => {[$(
                (
                    Delay::new(scale_delay($delay)),
                    FeedforwardComb::new(
                        0.8 * (10.0 as f64).powf(-3.0 * f64::from($delay) / (44100.0 * t60)), 0.2, scale_delay($delay)),
                ),
            )*]}
        }
        macro_rules! delays {
            ($($delay:expr),*) => {[$(
                Delay::new(scale_delay($delay)),
            )*]}
        }
        Self {
            allpasses: allpasses_from_delays![225, 341, 441],
            combs: combs_from_delays![1116, 1356, 1422, 1617],
            out_delays: delays![211, 179],
        }
    }
}

impl Reverb for STKJCRev {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64) {
        let input = (x.0 + x.1) / 2.0;

        let allpass_output = self
            .allpasses
            .iter_mut()
            .fold(input, |output, a| a.process_sample(output));

        let comb_output = self
            .combs
            .iter_mut()
            .map(|(delay, filter)| {
                let output = allpass_output + filter.process_sample(delay.output());
                delay.input(output);
                output
            })
            .sum();

        let output = (
            0.7 * (0.3 * self.out_delays[0].output() + 0.7 * input),
            0.7 * (0.3 * self.out_delays[1].output() + 0.7 * input),
        );

        self.out_delays[0].input(comb_output);
        self.out_delays[1].input(comb_output);

        output
    }
}
