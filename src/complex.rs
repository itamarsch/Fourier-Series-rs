use std::ops::{Add, AddAssign, Mul};

use nannou::prelude::Vec2;

#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f32,
    pub img: f32,
}

impl Into<Vec2> for Complex {
    fn into(self) -> Vec2 {
        Vec2::new(self.re, self.img)
    }
}

impl Complex {
    pub fn polar(r: f32, theta: f32) -> Complex {
        Complex {
            re: theta.cos() * r,
            img: theta.sin() * r,
        }
    }
    pub fn mag(self) -> f32 {
        f32::hypot(self.img, self.re)
    }
}

impl Add<Complex> for Complex {
    fn add(self, other: Complex) -> Self::Output {
        Complex {
            re: self.re + other.re,
            img: self.img + other.img,
        }
    }

    type Output = Complex;
}

impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, other: Complex) {
        let sum = other + *self;
        self.re = sum.re;
        self.img = sum.img;
    }
}

impl Mul<f32> for Complex {
    type Output = Complex;

    //Scale
    fn mul(self, n: f32) -> Self::Output {
        Complex {
            re: self.re * n,
            img: self.img * n,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    //Complex multiplication
    fn mul(self, other: Complex) -> Self::Output {
        Complex {
            re: self.re * other.re - self.img * other.img,
            img: self.re * other.img + self.img * other.re,
        }
    }
}
