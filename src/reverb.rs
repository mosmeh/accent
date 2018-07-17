use delay::Delay;
use filter::{Allpass, FeedbackComb, FeedforwardComb, Filter, LowpassFeedbackComb};

pub trait Reverb {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64);
}

// https://ccrma.stanford.edu/~jos/pasp/Schroeder_Reverberators.html
pub struct JCRev {
    allpasses: [Allpass; 3],
    combs: [FeedbackComb; 4],
}

impl JCRev {
    pub fn new(sample_rate: u32) -> Self {
        let sr_factor = f64::from(sample_rate) / 25000.0;
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, (sr_factor * f64::from($delay)) as usize),
            )*]}
        }
        macro_rules! combs_from_feedbacks_and_delays {
            ($($am:expr, $delay:expr);*) => {[$(
                FeedbackComb::new($am, (sr_factor * f64::from($delay)) as usize),
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

// https://ccrma.stanford.edu/software/stk/
// https://github.com/thestk/stk/blob/master/include/JCRev.h
pub struct STKJCRev {
    allpasses: [Allpass; 3],
    combs: [(Delay, FeedforwardComb); 4],
    out_delays: [Delay; 2],
}

impl STKJCRev {
    pub fn new(sample_rate: u32, t60: f64) -> Self {
        let sr_factor = f64::from(sample_rate) / 44100.0;
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, (sr_factor * f64::from($delay)) as usize),
            )*]}
        }
        macro_rules! combs_from_delays {
            ($($delay:expr),*) => {[$(
                (
                    Delay::new((sr_factor * f64::from($delay)) as usize),
                    FeedforwardComb::new(
                        0.8 * (10.0 as f64).powf(-3.0 * f64::from($delay) / (44100.0 * t60)),
                        0.2, (sr_factor * f64::from($delay)) as usize),
                ),
            )*]}
        }
        macro_rules! delays {
            ($($delay:expr),*) => {[$(
                Delay::new((sr_factor * f64::from($delay)) as usize),
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

// https://ccrma.stanford.edu/software/stk/
// https://github.com/thestk/stk/blob/master/include/PRCRev.h
pub struct PRCRev {
    allpasses: [Allpass; 2],
    combs: [FeedbackComb; 2],
}

impl PRCRev {
    pub fn new(sample_rate: u32, t60: f64) -> Self {
        let sr_factor = f64::from(sample_rate) / 44100.0;
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, (sr_factor * f64::from($delay)) as usize),
            )*]}
        }
        macro_rules! combs_from_delays {
            ($($delay:expr),*) => {[$(
                FeedbackComb::new(-(10.0 as f64).powf(-3.0 * f64::from($delay) / (44100.0 * t60)),
                    (sr_factor * f64::from($delay)) as usize),
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

// https://ccrma.stanford.edu/software/stk/
// https://github.com/thestk/stk/blob/master/include/NRev.h
pub struct NRev {
    fb_combs: [FeedbackComb; 6],
    ff_comb: FeedforwardComb,
    allpasses: [Allpass; 6],
}

impl NRev {
    pub fn new(sample_rate: u32, t60: f64) -> Self {
        let sr_factor = f64::from(sample_rate) / 25641.0;
        macro_rules! combs_from_delays {
            ($($delay:expr),*) => {[$(
                FeedbackComb::new(-(10.0 as f64).powf(-3.0 * f64::from($delay) / (44100.0 * t60)),
                    (sr_factor * f64::from($delay)) as usize),
            )*]}
        }
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, (sr_factor * f64::from($delay)) as usize),
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

// https://ccrma.stanford.edu/~jos/pasp/Example_Schroeder_Reverberators.html
pub struct SATREV {
    combs: [FeedbackComb; 4],
    allpasses: [Allpass; 3],
}

impl SATREV {
    pub fn new(sample_rate: u32) -> Self {
        let sr_factor = f64::from(sample_rate) / 25000.0;
        macro_rules! combs_from_feedbacks_and_delays {
            ($($am:expr, $delay:expr);*) => {[$(
                FeedbackComb::new($am, (sr_factor * f64::from($delay)) as usize),
            )*]}
        }
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                Allpass::new(-0.7, -0.7, (sr_factor * f64::from($delay)) as usize),
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
            wet1: (width / 2.0 + 0.5) * 3.0 * wet,
            wet2: (1.0 - width) / 2.0 * 3.0 * wet,
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
    lbcfs: [LowpassFeedbackComb; 8],
    allpasses: [(FeedbackComb, FeedforwardComb); 4],
}

impl MonoFreeverb {
    fn new(sample_rate: u32, feedback: f64, damp: f64, stereo_spread: u32) -> Self {
        const G: f64 = 0.5;
        let sr_factor = f64::from(sample_rate) / 44100.0;
        macro_rules! lbcfs_from_delays {
            ($($delay:expr),*) => {[$(
                LowpassFeedbackComb::new(feedback, damp,
                    (sr_factor * f64::from($delay + stereo_spread)) as usize),
            )*]}
        }
        macro_rules! allpasses_from_delays {
            ($($delay:expr),*) => {[$(
                (
                    FeedbackComb::new(-G, (sr_factor * f64::from($delay + stereo_spread)) as usize),
                    FeedforwardComb::new(-1.0, 1.0 + G,
                        (sr_factor * f64::from($delay + stereo_spread)) as usize)
                ),
            )*]}
        }
        Self {
            lbcfs: lbcfs_from_delays![1557, 1617, 1491, 1422, 1277, 1356, 1188, 1116],
            allpasses: allpasses_from_delays![225, 556, 441, 341],
        }
    }
}

impl Filter for MonoFreeverb {
    fn process_sample(&mut self, x: f64) -> f64 {
        let lbcf_output = self.lbcfs.iter_mut().map(|c| c.process_sample(x)).sum();

        self.allpasses
            .iter_mut()
            .fold(lbcf_output, |acc, (fbcf, ffcf)| {
                ffcf.process_sample(fbcf.process_sample(acc))
            })
    }
}
