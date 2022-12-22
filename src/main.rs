extern crate core;

mod fft;
mod freq_container;
mod fft_test;
mod plot;
mod bar_visualizer;
mod test_bar_visualizer;
mod analyzer;
mod cli;

use std::borrow::Borrow;
use std::collections::LinkedList;
use std::fs;
use clap::Parser;
use kdam::{BarExt, Column, RichProgress, tqdm};
use kdam::term::Colorizer;
use num_complex::{Complex, Complex32};
use opencv::core::{Mat, Size};
use opencv::highgui::{imshow, wait_key};
use opencv::sys::cv_VideoWriter_fourcc_char_char_char_char;
use opencv::videoio::{CAP_PROP_FOURCC, VideoWriter, VideoWriterTrait};
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use crate::bar_visualizer::{BarVisualizer, Visualizer};
use crate::cli::Args;
use crate::fft::FFT;
use crate::freq_container::{FrequencyContainer, LogFrequenciesContainer};
use crate::plot::plot_data;

fn populate_fft(v: &mut Vec<Complex32>, mut format: Box<dyn FormatReader>) {
    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                unimplemented!();
            }
            Err(_err) => {
                // file ends
                println!("{}", v.len());
                break;
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

            // Consume the new metadata at the head of the metadata queue.
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Consume the decoded audio samples (see below).
                match decoded {
                    AudioBufferRef::F32(buf) => {
                        for &sample in buf.chan(0) {
                            // Do something with `sample`.
                            v.push(Complex32::new(sample, 0.0));
                        }
                    }
                    _ => {
                        // Repeat for the different sample formats.
                        unimplemented!()
                    }
                }
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                continue;
            }
            Err(err) => {
                // An unrecoverable error occured, halt decoding.
                panic!("{}", err);
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let path = args.input_path();
    let temp_path = args.temp_path();
    let output_path = args.output_path();

    let output_resolution = (720, 220);

    // Open the media source.
    let src = fs::File::open(&path).expect("failed to open media");

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let mut format = probed.format;

    let mut samples: Vec<Complex32> = Vec::new();

    populate_fft(&mut samples, format);

    let mut nearest_power: usize = 1;
    while nearest_power < (1 << 10).min(samples.len()) {
        nearest_power <<= 1;
    }

    println!("nearest power: {}", nearest_power);

    let window_fps = 60;
    let bitrate = 48000;
    let fft_window_s = bitrate / window_fps as usize;

    let fourcc = VideoWriter::fourcc('m', 'p', '4', 'v').unwrap();
    let mut video_writer = VideoWriter::new(temp_path,
                                            fourcc,
                                            60.0, Size::from(output_resolution), true).unwrap();

    let mut progress_bar = RichProgress::new(
        tqdm!(total = samples.len(), unit_scale = true, unit_divisor = 1024, unit = "B"),
        vec![Column::Spinner(
            "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            80.0,
            1.0,
        ),
             Column::text("[bold blue]?"),
             Column::Bar,
             Column::Percentage(1),
             Column::text("•"),
             Column::CountTotal,
             Column::text("•"),
             Column::Rate,
             Column::text("•"),
             Column::RemainingTime],
    );

    progress_bar.write("Processing audio input...".colorize("bold cyan"));
    let vis = BarVisualizer::new(output_resolution.0 as usize,
                                 output_resolution.1 as usize,
                                 0);

    let mut analyzer = analyzer::AudioAnalyzer::new(
        1024,
        Some(Box::new(vis)),
        Box::new(LogFrequenciesContainer::new(50, 24000.0)),
        progress_bar);

    // how many times per second
    for t in 0..samples.len() {
        let value = samples.get(t).unwrap().to_owned();
        analyzer.push_audio(value.re);

        if t % fft_window_s == 0 && t > nearest_power {
            let frame = analyzer.get_frame();
            match frame {
                None => {}
                Some(f) => {
                    video_writer.write(&f).unwrap();
                }
            }
        }
    }

    analyzer.off();
    video_writer.release().unwrap();

    let mut command = std::process::Command::new("ffmpeg");
    command.arg("-i")
        .arg(temp_path.to_owned())
        .arg("-i")
        .arg(path.to_owned())
        .arg("-c")
        .arg("copy")
        .arg("-map")
        .arg("0:v:0")
        .arg("-y")
        .arg("-map")
        .arg("1:a:0")
        .arg(output_path);

    command.output().unwrap();
}