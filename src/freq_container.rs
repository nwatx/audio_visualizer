use std::fmt::{Display, Formatter};
use std::slice::Iter;

#[derive(Debug)]
pub struct LogFrequenciesContainer {
    buckets: Vec<f32>,
    bucket_count: usize,
    freq_range: f32,
    bucket_size: f32,
}

pub trait FrequencyContainer {
    fn update_frequency(&mut self, f: f32, cnt: f32);
    fn get_iter(&self) -> Iter<'_, f32>;
    fn num_buckets(&self) -> usize;
    fn clear(&mut self);
}

impl LogFrequenciesContainer {
    pub fn new(bucket_count: usize, mut freq_range: f32) -> LogFrequenciesContainer {
        freq_range = freq_range.log2();

        LogFrequenciesContainer {
            buckets: vec![0.0; bucket_count as usize],
            bucket_count,
            freq_range,
            bucket_size: (freq_range as f32 / bucket_count as f32),
        }
    }

    pub fn num_buckets(&self) -> usize {
        self.bucket_count
    }
}

impl FrequencyContainer for LogFrequenciesContainer {
    fn update_frequency(&mut self, mut f: f32, cnt: f32) {
        let scaling_factor = self.num_buckets() as f32 / self.freq_range;
        let idx = ((f / self.freq_range + 1.0).log2() * scaling_factor) as usize;
        // eprintln!("f: {}, cnt: {}, idx: {}", f, cnt, idx);
        if (0..self.bucket_count).contains(&idx) {
            self.buckets[idx] += cnt;
        }
    }

    fn get_iter(&self) -> Iter<'_, f32> {
        self.buckets.iter()
    }

    fn num_buckets(&self) -> usize {
        self.bucket_count
    }

    fn clear(&mut self) {
        for i in 0..self.bucket_count {
            self.buckets[i] = 0.0;
        }
    }
}

impl Display for LogFrequenciesContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // build the buckets
        write!(f, "{:?}", self.buckets)
    }
}