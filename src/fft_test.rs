use rand::Rng;

#[cfg(test)]
mod fft_test {
    use std::f32::consts::PI;
    use itertools_num::linspace;
    use num_complex::{Complex, Complex32};
    use plotlib::page::Page;
    use plotlib::repr::{ContinuousRepresentation, Plot};
    use plotlib::style::{LineStyle, PointMarker, PointStyle};
    use plotlib::view::ContinuousView;
    use rand::{Rng, thread_rng};
    use rand::distributions::Uniform;
    use crate::fft::FFT;
    use crate::plot;


    #[test]
    fn test_sin() {
        let mut rng = thread_rng();
        let mut sample_data: Vec<f32> = Vec::new();
        let mut sample_data_plot: Vec<(f64, f64)> = Vec::new();

        const N: i32 = 1080;
        const T: f64 = 1.0 / (N as f64);

        for t in 0..N {
            let angle = (t as f32 * 2.0 * PI * T as f32) as f32;
            sample_data.push(angle.sin());
            sample_data_plot.push((angle as f64, angle.sin() as f64));
        }

        plot::plot_data(sample_data_plot, String::from("/home/neo/CLionProjects/audio_visualizer/data/scatter1.svg"));

        println!("{:?}", sample_data);

        let resolution = 1;

        FFT::calculate_frequencies(&sample_data);

        // let f: Vec<_> = sample_data.iter().map(|x| x.atan().re * 180.0 / PI).collect();
        // println!("{:?}", f);

        let mut f: Vec<(f64, f64)> = Vec::new();
        let mut cnt = 0;

        let s_len = sample_data.len();

        for x in sample_data {
            // println!("magnitude: {}, angle: {}", x.norm(), x.atan().re);
            // f.push((x.norm() as f64, x.atan().re as f64));
            // f.push((cnt as f64, (x.norm() as f64)));
            cnt += 1;
        }

        plot::plot_data(f, String::from("/home/neo/CLionProjects/audio_visualizer/data/scatter2.svg"));
    }

    #[test]
    fn test_scipy() {
        const N: i32 = 800;
        const T: f32 = 1.0 / 800.0;
        // x = np.linspace(0.0, N*T, N, endpoint=False)
        // y = np.sin(50.0 * 2.0*np.pi*x) + 0.5*np.sin(80.0 * 2.0*np.pi*x)
        // yf = fft(y)
        // xf = fftfreq(N, T)[:N//2]
        // import matplotlib.pyplot as plt
        // plt.plot(xf, 2.0/N * np.abs(yf[0:N//2]))
        // plt.grid()
        let x: Vec<f32> = linspace(0., (N as f32) * T, N as usize).map(|x| x as f32).collect();
        // let y: Vec<f32> = x.iter().map(|x| (50.0 * 2.0 * (PI as f64) * x).sin() as f64 + 0.5 * (80.0 * 2.0 * (PI as f64) * x).sin() as f64).collect();
        // let mut yf: Vec<_> = y.iter().map(|y| Complex32::new(y.to_owned() as f32, 0.0)).collect();
        // FFT::calculate_frequencies(&y);
        //
        // let to_plot: Vec<(f64, f64)> = yf.iter().enumerate().map((|(i, x)| (i as f64, x.norm() as f64))).collect();
        // plot::plot_data(to_plot, String::from("/home/neo/CLionProjects/audio_visualizer/data/scatter2.svg"));
        //
        // println!("{:?}", y);
        //
        // let to_plot: Vec<(f64, f64)> = y.iter().enumerate().map((|(i, x)| (i as f64, x.to_owned() as f64))).collect();
        // plot::plot_data(to_plot, String::from("/home/neo/CLionProjects/audio_visualizer/data/scatter1.svg"));
    }
}