#![deny(bare_trait_objects)]

extern crate accent;
extern crate clap;
extern crate failure;
extern crate hound;
extern crate itertools;

use accent::*;
use clap::{App, Arg, SubCommand};
use failure::{err_msg, Error};
use hound::{Sample, SampleFormat, WavReader, WavSpec, WavWriter};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let arg_input = Arg::with_name("input")
        .help("Input WAV filename")
        .required(true)
        .index(1);
    let app_m = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Output WAV filename")
                .takes_value(true)
                .default_value("out.wav")
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("jcrev")
                .about("Original JCRev")
                .arg(&arg_input),
        )
        .subcommand(
            SubCommand::with_name("stk-jcrev")
                .about("JCRev in Synthesis ToolKit")
                .arg(&arg_input)
                .arg(Arg::with_name("t60").long("t60").default_value("1")),
        )
        .subcommand(
            SubCommand::with_name("prcrev")
                .about("PRCRev")
                .arg(&arg_input)
                .arg(Arg::with_name("t60").long("t60").default_value("1")),
        )
        .subcommand(
            SubCommand::with_name("satrev")
                .about("SATREV")
                .arg(&arg_input),
        )
        .subcommand(
            SubCommand::with_name("freeverb")
                .about("Freeverb")
                .arg(&arg_input)
                .arg(
                    Arg::with_name("roomsize")
                        .long("roomsize")
                        .default_value("0.1"),
                )
                .arg(Arg::with_name("damp").long("damp").default_value("0.1"))
                .arg(Arg::with_name("width").long("width").default_value("1"))
                .arg(Arg::with_name("wet").long("wet").default_value("1"))
                .arg(Arg::with_name("dry").long("dry").default_value("0")),
        )
        .get_matches();
    let input = match app_m.subcommand() {
        (_, Some(sub_m)) => sub_m.value_of("input").unwrap(),
        _ => return Err(err_msg("Reveberation algorithm were not provided")),
    };
    let output = app_m.value_of("output").unwrap();

    let mut reader = WavReader::open(input)?;
    let input_channels = reader.spec().channels;
    let sample_rate = reader.spec().sample_rate;

    let mut reverb: Box<dyn Reverb> = match app_m.subcommand() {
        ("jcrev", Some(_)) => Box::new(JCRev::new(sample_rate)),
        ("stk-jcrev", Some(sub_m)) => Box::new(STKJCRev::new(
            sample_rate,
            sub_m.value_of("t60").unwrap().parse()?,
        )),
        ("prcrev", Some(sub_m)) => Box::new(PRCRev::new(
            sample_rate,
            sub_m.value_of("t60").unwrap().parse()?,
        )),
        ("satrev", Some(_)) => Box::new(SATREV::new(sample_rate)),
        ("freeverb", Some(sub_m)) => Box::new(Freeverb::new(
            sample_rate,
            sub_m.value_of("roomsize").unwrap().parse()?,
            sub_m.value_of("damp").unwrap().parse()?,
            sub_m.value_of("width").unwrap().parse()?,
            sub_m.value_of("wet").unwrap().parse()?,
            sub_m.value_of("dry").unwrap().parse()?,
        )),
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
