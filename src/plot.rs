use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;

pub fn plot_data(v: Vec<(f64, f64)>, file_path: String) {
    let figure_1 = Plot::new(v).line_style(
        LineStyle::new()
        // .marker(PointMarker::Circle) // setting the marker to be a square
        // .size(1.0)
        // .colour("#DD3355"),
    );

    let v = ContinuousView::new()
        .add(figure_1)
        .y_range(0.0, 20.0)
        .x_label("Some varying variable")
        .y_label("The response of something");

    Page::single(&v).save(file_path).unwrap();
}
