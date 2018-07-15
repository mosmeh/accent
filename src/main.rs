#![deny(bare_trait_objects)]

extern crate accent;
extern crate clap;
extern crate hound;
extern crate itertools;

use accent::*;
use clap::{App, Arg};
use hound::{Error, Sample, SampleFormat, WavReader, WavSpec, WavWriter};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("input")
                .help("Input WAV filename")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Output WAV filename")
                .takes_value(true)
                .default_value("out.wav"),
        )
        .arg(
            Arg::with_name("algorithm")
                .short("a")
                .help("Reverberation algorithm")
                .takes_value(true)
                .default_value("jcrev")
                .possible_values(&["jcrev"]),
        )
        .get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let mut reader = WavReader::open(input)?;
    let input_channels = reader.spec().channels;
    let sample_rate = reader.spec().sample_rate;

    let mut reverb: Box<dyn Reverb> = match matches.value_of("algorithm").unwrap() {
        "jcrev" => Box::new(JCRev::new(sample_rate)),
        _ => unreachable!(),
    };

    let samples = reader
        .samples::<i16>()
        .map(|s| f64::from(s.unwrap().as_i16()));
    let stereo_samples: Vec<_> = match input_channels {
        1 => samples.map(|s| (s, s)).collect(),
        2 => samples
            .chunks(2)
            .into_iter()
            .map(|mut c| c.next_tuple::<(f64, f64)>().unwrap())
            .collect(),
        _ => unimplemented!(),
    };

    let write_spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(output, write_spec)?;

    for x in stereo_samples {
        let (l, r) = reverb.process_sample(x);
        writer.write_sample(l as i16)?;
        writer.write_sample(r as i16)?;
    }

    writer.finalize()?;

    Ok(())
}
