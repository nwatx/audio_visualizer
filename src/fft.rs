use std::collections::{LinkedList, VecDeque};
use num_complex::Complex32;
use rustfft::{FftPlanner, num_complex::Complex};
use spectrum_analyzer::{FrequencyLimit, FrequencySpectrum, samples_fft_to_spectrum};

pub struct FFT {}

impl FFT {
    pub fn calculate_frequencies(v: &[f32]) -> FrequencySpectrum {
        // let mut planner: FftPlanner<f32> = FftPlanner::new();
        // let mut v2 = v.clone();
        // let fft = planner.plan_fft_forward(v2.len());
        // fft.process(&mut v2);

        let spectrum = samples_fft_to_spectrum(v,
                                               48000,
                                               FrequencyLimit::All,
                                               None).unwrap();

        return spectrum;
        // for (fr, fr_val) in spectrum.data().iter() {
        //     println!("{}Hz => {}", fr, fr_val)
        // }
        // println!("{:?}", v2);
    }
}