use filter::{Allpass, FeedbackComb};

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
        Self {
            allpasses: [
                Allpass::new(-0.7, -0.7, (sr_factor * 347.0) as usize),
                Allpass::new(-0.7, -0.7, (sr_factor * 113.0) as usize),
                Allpass::new(-0.7, -0.7, (sr_factor * 37.0) as usize),
            ],
            combs: [
                FeedbackComb::new(0.773, (sr_factor * 1687.0) as usize),
                FeedbackComb::new(0.802, (sr_factor * 1601.0) as usize),
                FeedbackComb::new(0.753, (sr_factor * 2053.0) as usize),
                FeedbackComb::new(0.733, (sr_factor * 2251.0) as usize),
            ],
        }
    }
}

impl Reverb for JCRev {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64) {
        let mut allpass_output = (x.0 + x.1) / 2.0;
        for allpass in &mut self.allpasses.iter_mut() {
            allpass_output = allpass.process_sample(allpass_output);
        }

        let comb_output: Vec<_> = self.combs
            .iter_mut()
            .map(|c| c.process_sample(allpass_output))
            .collect();

        (
            comb_output[0] + comb_output[2],
            comb_output[1] + comb_output[3],
        )
    }
}
