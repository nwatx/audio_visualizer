use std::borrow::Borrow;
use std::collections::VecDeque;
use kdam::{BarExt, RichProgress};
use opencv::core::Mat;
use spectrum_analyzer::FrequencySpectrum;
use crate::bar_visualizer::Visualizer;
use crate::fft::FFT;
use crate::freq_container::FrequencyContainer;

pub struct AudioAnalyzer {
    buffer_size: usize,
    visualizer: Option<Box<dyn Visualizer>>,
    progress_bar: RichProgress,
    frequencies: Box<dyn FrequencyContainer>,
    buffer: VecDeque<f32>,
    cnt: usize,
}

impl AudioAnalyzer {
    /// Creates a new audio analyzer with a buffer sized to the nearest power of 2
    pub fn new(buffer_size: usize,
               visualizer: Option<Box<dyn Visualizer>>,
               frequency_container: Box<dyn FrequencyContainer>,
               progress_bar: RichProgress) -> AudioAnalyzer {
        // set the buffer_size to the nearest power of 2
        let mut p2 = 1;
        while p2 < buffer_size {
            p2 <<= 1;
        }

        AudioAnalyzer {
            buffer_size: p2,
            visualizer,
            frequencies: frequency_container,
            buffer: VecDeque::new(),
            progress_bar,
            cnt: 0,
        }
    }

    /// Adds the byte to the current buffer and resizes accordingly
    fn maintain_buffer(&mut self, byte: f32) {
        self.buffer.push_back(byte);

        if self.buffer.len() > self.buffer_size {
            self.buffer.pop_front();
        }
    }

    /// Updates the progress bar, if it exists
    fn update_progress(&mut self) {
        self.cnt += 1;
        self.progress_bar.update_to(self.cnt);
    }

    /// calculates frequencies of the fft
    fn calculate_frequencies(&self) -> FrequencySpectrum {
        let mut binding = self.buffer.clone();
        let buffer = binding.make_contiguous();
        FFT::calculate_frequencies(buffer)
    }

    fn update_frequencies(&mut self) {
        self.frequencies.clear();
        let freq_calc = self.calculate_frequencies();
        for (hz, cnt) in freq_calc.data().iter() {
            self.frequencies.update_frequency(hz.val(), cnt.val());
        }
    }

    /// calls a new byte
    pub fn push_audio(&mut self, byte: f32) {
        self.update_progress();
        self.maintain_buffer(byte);
    }

    /// gets the current frame
    pub fn get_frame(&mut self) -> Option<Mat> {
        let has_vis = !self.visualizer.is_none();
        if has_vis {
            self.update_frequencies();
            Some(self.visualizer.as_ref().unwrap().next_frame(self.frequencies.borrow()))
        } else {
            None
        }
    }

    pub fn off(&self) {
        let has_vis = !self.visualizer.is_none();
        if has_vis {
            let vis = self.visualizer.as_ref().unwrap();
            (*vis).off();
        }
    }
}