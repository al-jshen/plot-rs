use nannou::prelude::*;
use plot_rs::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.main_window().rect();

    draw.background().color(WHITESMOKE);

    Axis::default().draw(&draw, &win);
    Xticks::default().draw(&draw, &win);
    Yticks::default().draw(&draw, &win);

    let f = |x: f32| 50. * (1. / 10. * x).cos() + 100. * (1. / 5. * x).sin();

    let points = (-400..400).into_iter().map(|x| (x as f32, f(x as f32)));

    draw.polyline().weight(2.).color(BLUE).points(points);

    draw.to_frame(app, &frame).unwrap();
}
