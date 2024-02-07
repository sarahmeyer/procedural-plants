use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    const LEAF_HEIGHT: f32 = 15.0;
    const LEAF_WIDTH: f32 = 11.0;
    const LEAVES_NUM: i32 = 120;
    const VINE_WAVE_SCALE: f32 = 20.0;
    const STEM_LENGTH: f32 = 20.0;

    frame.clear(WHITE);
    let draw = app.draw();

    let mut rng = rand::thread_rng();

    // sin curve
    let vine_points = (0..50).map(|i| {
        let x = i as f32 - 25.0; //subtract 25 to center the sine wave
        let point = pt2(x, x.sin()) * VINE_WAVE_SCALE; //scale sine wave by 20.0
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(vine_points);

    // let filtered_points = points.clone().filter(|p| p.0.x == 0.0);
    let leaf_origin_points = (0..LEAVES_NUM).map(|i| {
        let x = i as f32 - 25.0;
        pt2(x, x.sin()) * VINE_WAVE_SCALE
        // pt2(x * std::f64::consts::PI as f32, x.sin()) * VINE_WAVE_SCALE
    });
    for point in leaf_origin_points {
        // find closest point to this x in vine_points array and draw a line segment
        let point_on_vine_curve = pt2(point.x, point.y);

        let random_x: f32 = rng.gen();
        let random_y: f32 = rng.gen();
        let leaf_center = Point2::new(
            point.x + STEM_LENGTH * random_x,
            point.y + STEM_LENGTH * random_y,
        );
        draw.line()
            .start(leaf_center)
            .end(point_on_vine_curve)
            .weight(2.0)
            .color(GREEN);
        draw.ellipse()
            .color(GREEN)
            .w(LEAF_WIDTH)
            .h(LEAF_HEIGHT)
            .xy(leaf_center);
    }

    // let points.filter(|p| p[0] % p[0].floor() > );

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
