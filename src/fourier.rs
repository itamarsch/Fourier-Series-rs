use std::collections::HashMap;

use super::Complex;
use lerp::Lerp;
use nannou::prelude::*;

const AMOUNT_OF_ARROOWS: i32 = 300;
const DT_INTEGRAL: f32 = 0.00001;
fn calculate_nth_c(f: &'static [Complex], frequency: i32) -> Complex {
    let mut sum = Complex { re: 0.0, img: 0.0 };
    let mut t = DT_INTEGRAL;
    while t <= 1.0 {
        let current_f: Complex = access_drawing_at_t(f, t);
        let c = Complex::polar(1.0, 2.0 * PI * (frequency as f32) * t * -1.0);
        sum = sum.add(c.mult(current_f).scale(DT_INTEGRAL));
        t += DT_INTEGRAL;
    }

    sum
}

fn access_drawing_at_t(f: &'static [Complex], t: f32) -> Complex {
    let index = Lerp::lerp(0.0, (f.len() - 1) as f32, t).floor() as usize;
    let current_f = f.get(index).unwrap();
    let next_f = f.get(index + 1).unwrap_or(current_f);

    let lerped_current_f = current_f.lerp(*(next_f), t);

    lerped_current_f
}

pub fn calculate_arrows(cs: &HashMap<i32, Complex>, t: f32) -> (Vec<Complex>, Complex) {
    let mut sum = Complex { re: 0.0, img: 0.0 };
    (
        (1..=AMOUNT_OF_ARROOWS)
            .map(|v| vec![v, -v])
            .flatten()
            .map(|v| {
                let res = cs
                    .get(&v)
                    .unwrap()
                    .mult(Complex::polar(1.0, 2.0 * PI * v as f32 * t));
                sum = sum.add(res);
                res
            })
            .collect(),
        sum,
    )
}

pub fn arrows(draw: &Draw, waves: &Vec<Complex>) {
    let mut sum = Complex { re: 0.0, img: 0.0 };
    waves.iter().for_each(|v| {
        let prev = sum;
        sum = sum.add(*(v));

        // draw.ellipse()
        //     .width(v.mag() * 2.0)
        //     .height(v.mag() * 2.0)
        //     .x_y(prev.re, prev.img)
        //     .no_fill()
        //     .stroke_color(WHITE)
        //     .stroke_weight(0.05);

        draw.arrow()
            .start(prev.into())
            .end(sum.into())
            .head_length(v.mag() * 0.15)
            .head_width(8.0.to_radians().tan() * v.mag() * 0.85)
            .color(WHITE)
            .weight(0.6);
    });
}

pub fn calculate_cs(drawing: &'static [Complex]) -> HashMap<i32, Complex> {
    (-AMOUNT_OF_ARROOWS..=AMOUNT_OF_ARROOWS)
        .map(|n| (n, calculate_nth_c(drawing, n)))
        .collect()
}
