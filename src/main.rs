use complex::Complex;
use drawings::fourier_portrait;
use nannou::prelude::*;
use std::collections::HashMap;

mod complex;
mod drawings;
mod fourier;

const DT: f32 = 0.001;
fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    t: f32,
    cs: HashMap<i32, Complex>,
    path: Vec<Complex>,
    arrows: Vec<Complex>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .maximized(true)
        .title("fourier")
        .build()
        .unwrap();
    Model {
        _window,
        t: DT,
        cs: fourier::calculate_cs(Vec::from(fourier_portrait())),
        path: Vec::new(),
        arrows: Vec::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.t >= 1.0 {
        model.t = DT;
    } else {
        let (arrows, sum) = fourier::calculate_arrows(&model.cs, model.t);
        model.arrows = arrows;
        model.path.push(sum);
        model.t += DT;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    fourier::arrows(&draw, &model.arrows);

    draw.polyline()
        .weight(3.0)
        .points(
            model
                .path
                .iter()
                .map(|v| -> Vec2 { Complex::into(v.clone()) }),
        )
        .color(GREEN);

    draw.to_frame(app, &frame).unwrap()
}
