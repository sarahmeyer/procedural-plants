use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
    let draw = app.draw();

    // let mut rng = rand::thread_rng();
    //     let random_u: f64 = rng.gen();

    // Draw a blue ellipse with default size and position.
    draw.ellipse().color(STEELBLUE);

    let points = (0..50).map(|i| {
        let x = i as f32 - 25.0; //subtract 25 to center the sine wave
        let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
