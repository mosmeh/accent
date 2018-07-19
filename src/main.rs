#![deny(bare_trait_objects)]

extern crate accent;
extern crate clap;
extern crate hound;
extern crate itertools;

use accent::*;
use clap::{App, AppSettings, Arg, SubCommand};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_input = Arg::with_name("input")
        .help("Input WAV file")
        .required(true)
        .index(1);
    let app_m = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Output WAV file")
                .default_value("out.wav")
                .takes_value(true)
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("jcrev")
                .about("Original JCRev")
                .arg(&arg_input),
        )
        .subcommand(
            SubCommand::with_name("satrev")
                .about("SATREV")
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
            SubCommand::with_name("nrev")
                .about("NRev")
                .arg(&arg_input)
                .arg(Arg::with_name("t60").long("t60").default_value("1")),
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
        _ => unreachable!(),
    };
    let output = app_m.value_of("output").unwrap();

    let mut reader = WavReader::open(input)?;
    let input_channels = reader.spec().channels;
    let sample_rate = reader.spec().sample_rate;

    let mut reverb: Box<dyn Reverb> = match app_m.subcommand() {
        ("jcrev", Some(_)) => Box::new(JCRev::new(sample_rate)),
        ("satrev", Some(_)) => Box::new(SATREV::new(sample_rate)),
        ("stk-jcrev", Some(sub_m)) => Box::new(STKJCRev::new(
            sample_rate,
            sub_m.value_of("t60").unwrap().parse()?,
        )),
        ("prcrev", Some(sub_m)) => Box::new(PRCRev::new(
            sample_rate,
            sub_m.value_of("t60").unwrap().parse()?,
        )),
        ("nrev", Some(sub_m)) => Box::new(NRev::new(
            sample_rate,
            sub_m.value_of("t60").unwrap().parse()?,
        )),
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

    let samples: Vec<_> = match reader.spec().sample_format {
        SampleFormat::Int => match reader.spec().bits_per_sample {
            16 => reader
                .samples::<i16>()
                .map(|s| f32::from(s.unwrap()) / f32::from(std::i16::MAX))
                .collect(),
            32 => reader
                .samples::<i32>()
                .map(|s| s.unwrap() as f32 / (std::i32::MAX as f32))
                .collect(),
            _ => unimplemented!(),
        },
        SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
    };
    let stereo_samples: Vec<_> = match input_channels {
        1 => samples.iter().map(|s| (s, s)).collect(),
        2 => samples
            .iter()
            .chunks(2)
            .into_iter()
            .map(|mut c| c.next_tuple().unwrap())
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

    for (l_in, r_in) in stereo_samples {
        let (l_out, r_out) = reverb.process_sample((f64::from(*l_in), f64::from(*r_in)));
        writer.write_sample((f64::from(std::i16::MAX) * l_out) as i16)?;
        writer.write_sample((f64::from(std::i16::MAX) * r_out) as i16)?;
    }

    writer.finalize()?;

    Ok(())
}
