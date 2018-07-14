extern crate accent;
extern crate clap;
extern crate hound;
extern crate itertools;

use accent::*;
use clap::{App, Arg};
use hound::{Sample, SampleFormat, WavReader, WavSpec, WavWriter};
use itertools::Itertools;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .index(1)
                .help("Input filename"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .help("Output filename"),
        )
        .arg(
            Arg::with_name("algorithm")
                .short("a")
                .takes_value(true)
                .help("Reverberation algorithm")
                .possible_values(&["jcrev"])
                .default_value("jcrev"),
        )
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("output").unwrap_or("out.wav");

    let mut reader = WavReader::open(input).expect("Failed to open input file");
    let input_channels = reader.spec().channels;
    let sample_rate = reader.spec().sample_rate;

    let write_spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(output, write_spec).expect("Failed to open output file");

    let mut reverb = Box::new(match matches.value_of("algorithm").unwrap() {
        "jcrev" => JCRev::new(sample_rate),
        _ => unreachable!(),
    });

    let samples = reader
        .samples::<i16>()
        .map(|s| f64::from(s.unwrap().as_i16()));
    let stereo_samples: Vec<(f64, f64)> = match input_channels {
        1 => samples.map(|s| (s, s)).collect(),
        2 => samples
            .chunks(2)
            .into_iter()
            .map(|mut c| c.next_tuple::<(f64, f64)>().unwrap())
            .collect(),
        _ => unimplemented!(),
    };

    for x in stereo_samples {
        let (l, r) = reverb.process_sample(x);
        writer.write_sample(l as i16).unwrap();
        writer.write_sample(r as i16).unwrap();
    }

    writer.finalize().expect("Failed to finalize");
}
