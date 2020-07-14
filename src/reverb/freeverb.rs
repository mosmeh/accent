use super::Reverb;
use crate::filter::{FeedbackComb, FeedforwardComb, Filter, LowpassFeedbackComb};

// https://ccrma.stanford.edu/~jos/pasp/Freeverb.html
// http://freeverb3vst.osdn.jp/
pub struct Freeverb {
    monos: [MonoFreeverb; 2],
    wet1: f64,
    wet2: f64,
    dry: f64,
}

impl Freeverb {
    pub fn new(sample_rate: u32, roomsize: f64, damp: f64, width: f64, wet: f64, dry: f64) -> Self {
        let feedback = 0.28 * roomsize + 0.7;
        Self {
            monos: [
                MonoFreeverb::new(sample_rate, feedback, damp, 0),
                MonoFreeverb::new(sample_rate, feedback, damp, 23),
            ],
            wet1: 1.5 * wet * (1.0 + width),
            wet2: 1.5 * wet * (1.0 - width),
            dry: 2.0 * dry,
        }
    }
}

impl Reverb for Freeverb {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64) {
        let input = 0.015 * (x.0 + x.1);
        let out = (
            self.monos[0].process_sample(input),
            self.monos[1].process_sample(input),
        );
        (
            self.wet1 * out.0 + self.wet2 * out.1 + self.dry * x.0,
            self.wet1 * out.1 + self.wet2 * out.0 + self.dry * x.1,
        )
    }
}

struct MonoFreeverb {
    lfbcs: [LowpassFeedbackComb; 8],
    allpasses: [(FeedbackComb, FeedforwardComb); 4],
}

impl MonoFreeverb {
    fn new(sample_rate: u32, feedback: f64, damp: f64, stereo_spread: u32) -> Self {
        let scale_delay = |d| (f64::from(sample_rate) / 44100.0 * f64::from(d)) as usize;
        macro_rules! lfbcs_from_delays {
            ($($delay:expr),*) => {[$(
                LowpassFeedbackComb::new(feedback, damp, scale_delay($delay + stereo_spread)),
            )*]}
        }
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                (
                    FeedbackComb::new(-0.5, scale_delay($delay + stereo_spread)),
                    FeedforwardComb::new(-1.0, 1.5, scale_delay($delay + stereo_spread)),
                ),
            )*]}
        }
        Self {
            lfbcs: lfbcs_from_delays![1557, 1617, 1491, 1422, 1277, 1356, 1188, 1116],
            allpasses: allpasses_from_delays![225, 556, 441, 341],
        }
    }
}

impl Filter for MonoFreeverb {
    fn process_sample(&mut self, x: f64) -> f64 {
        let lfbc_output = self.lfbcs.iter_mut().map(|c| c.process_sample(x)).sum();

        self.allpasses
            .iter_mut()
            .fold(lfbc_output, |acc, (fbcf, ffcf)| {
                ffcf.process_sample(fbcf.process_sample(acc))
            })
    }
}
