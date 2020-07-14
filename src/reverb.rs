mod freeverb;
mod jcrev;
mod nrev;
mod prcrev;
mod satrev;
mod stk_jcrev;

pub use freeverb::Freeverb;
pub use jcrev::JCRev;
pub use nrev::NRev;
pub use prcrev::PRCRev;
pub use satrev::SATREV;
pub use stk_jcrev::STKJCRev;

pub trait Reverb {
    fn process_sample(&mut self, x: (f64, f64)) -> (f64, f64);
}
