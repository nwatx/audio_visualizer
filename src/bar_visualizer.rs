use std::borrow::Borrow;
use opencv::core::{CV_8UC3, Mat, Rect, Scalar, Size};
use opencv::highgui::{destroy_all_windows, destroy_window, imshow, named_window, wait_key, WINDOW_AUTOSIZE};
use opencv::imgproc;
use opencv::imgproc::rectangle;
use crate::freq_container::{FrequencyContainer, LogFrequenciesContainer};

pub trait Visualizer {
    fn on(&self);
    fn off(&self);
    fn next_frame(&self, frequencies: &dyn FrequencyContainer) -> Mat;
}

pub struct BarVisualizer {
    resolution: (usize, usize),
    bucket_separation: usize,
}

impl BarVisualizer {
    pub fn new(width: usize, height: usize, bucket_separation: usize) -> BarVisualizer {
        BarVisualizer {
            resolution: (width, height),
            bucket_separation,
        }
    }

    pub fn width(&self) -> usize { self.resolution.0 }
    pub fn height(&self) -> usize { self.resolution.1 }
}

impl Visualizer for BarVisualizer {
    fn on(&self) {
        // named_window("test", WINDOW_AUTOSIZE).unwrap();
    }

    fn off(&self) {
        destroy_all_windows().unwrap();
    }

    fn next_frame(&self, frequencies: &dyn FrequencyContainer) -> Mat {
        let mut mat: Mat;
        unsafe {
            mat = Mat::new_size(Size::new(self.width() as i32, self.height() as i32), CV_8UC3).unwrap();
            // mat = Mat::new_rows_cols(self.resolution.1 as i32, self.resolution.0 as i32, CV_8UC3).unwrap();
        }

        // divide the buckets across mat
        // n * (bucket_width) + (n - 1) bucket_separation = width
        let buckets = frequencies.num_buckets();
        // println!("buckets {}, width: {}", buckets, self.width());
        // println!("{} {} {}", self.width(), buckets, self.bucket_separation);
        let bucket_width = (self.width() - (buckets - 1) * self.bucket_separation) / buckets;

        // build the height
        // frequencies should be normalized between 0, 1
        // TODO: remove this hardcoded constant
        let bar_max_height = 200.0;
        let bar_scale = self.height() as f64 / bar_max_height;

        rectangle(&mut mat,
                  Rect { x: 0, y: 0, width: self.width() as i32, height: self.height() as i32 },
                  Scalar::new(0.0, 0.0, 0.0, 255.0), -1, 8, 0).unwrap();

        for (i, bucket) in frequencies.get_iter().enumerate() {
            let bar_height = ((1.0 + bucket).clamp(0.0, bar_max_height as f32) * bar_scale as f32).max(5.0);
            let lower_left = (i * (bucket_width + self.bucket_separation), self.height() - bar_height as usize);
            // let top_right = (i * (bucket_width + self.bucket_separation) + bucket_width, bar_height as usize);

            let to_draw = Rect {
                x: lower_left.0 as i32,
                y: lower_left.1 as i32,
                width: bucket_width as i32,
                height: bar_height as i32,
            };

            rectangle(&mut mat, to_draw, Scalar::new(255.0, 255.0, 255.0, 255.0), 1, 8, 0).unwrap();
        }

        // TODO: if preview
        // imshow("test", &mat).unwrap();
        // wait_key(0).unwrap();
        // destroy_window("test").unwrap();

        mat
    }
}



