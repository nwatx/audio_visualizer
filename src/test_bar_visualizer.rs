#[cfg(test)]
mod test_bar_vis {
    use opencv::highgui::{imshow, wait_key};
    use crate::bar_visualizer::{BarVisualizer, Visualizer};
    use crate::freq_container::LogFrequenciesContainer;

    #[test]
    fn test_display_bar_visualizer() {
        let mut fc = LogFrequenciesContainer::new(10, 24000.0);
        fc.update_frequency(12000.0, 2000.0);
        fc.update_frequency(3000.0, 1.0);

        eprintln!("{:?}", fc);

        let bv = BarVisualizer::new(720, 480, 0);
        unsafe {
            imshow("test", &bv.next_frame(fc)).unwrap();
            wait_key(0).unwrap();
        }
    }
}