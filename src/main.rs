use complex::Complex;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

use std::collections::{HashMap, VecDeque};

mod complex;
mod drawings;
mod fourier;
mod lerp;

const DT: f32 = 0.001;
fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    egui: Egui,
    _window: window::Id,
    t: f32,
    cs: HashMap<i32, Complex>,
    path: VecDeque<Complex>,
    arrows: Vec<Complex>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .maximized(true)
        .raw_event(raw_window_event)
        .title("fourier")
        .build()
        .unwrap();

    let egui = Egui::from_window(&app.window(_window).unwrap());
    Model {
        egui,
        _window,
        t: DT,
        cs: fourier::calculate_cs(drawings::FOURIER_PORTRAIT),
        path: VecDeque::new(),
        arrows: Vec::new(),
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.egui.set_elapsed_time(_update.since_start);
    let ctx = model.egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Drawing");

        ui.horizontal(|ui| {
            for drawing in drawings::DRAWINGS {
                let btn = ui.button(drawing.title);
                if btn.clicked() {
                    model.cs = fourier::calculate_cs(drawing.points);
                    model.path.clear();
                    model.t = DT;
                }
            }
        });

        ui.label("Actions");
        let reset = ui.button("Reset");
        if reset.clicked() {
            model.t = DT;
            let (arrows, _) = fourier::calculate_arrows(&model.cs, model.t);
            model.arrows = arrows;
            model.path.clear()
        }
    });

    if model.path.len() > (1.0 / DT) as usize {
        model.path.pop_front();
    }

    if model.t >= 1.0 {
        model.t = DT;
    } else {
        let (arrows, sum) = fourier::calculate_arrows(&model.cs, model.t);
        model.arrows = arrows;
        model.path.push_back(sum);
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
                .map(|v| -> Vec2 { Vec2::new(v.re, v.img) }),
        )
        .color(GREEN);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
