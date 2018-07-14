extern crate accent;
extern crate clap;
extern crate hound;

use accent::*;
use clap::{App, Arg};

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

    let mut reader = hound::WavReader::open(input).expect("Failed to open input file");
    if reader.spec().channels > 1 {
        panic!("Mono input expected");
    }
    let sample_rate = reader.spec().sample_rate;

    let write_spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer =
        hound::WavWriter::create(output, write_spec).expect("Failed to open output file");

    let mut reverb = Box::new(match matches.value_of("algorithm").unwrap() {
        "jcrev" => JCRev::new(sample_rate),
        _ => unreachable!(),
    });

    for x in reader.samples::<i16>() {
        let s: f64 = x.unwrap().into();
        let (l, r) = reverb.process_sample((s, s));
        writer.write_sample(l as i16).unwrap();
        writer.write_sample(r as i16).unwrap();
    }

    writer.finalize().expect("Failed to finalize");
}
