use lerp::Lerp;
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
    pub fn add(self, other: Complex) -> Complex {
        Complex {
            re: other.re + self.re,
            img: other.img + self.img,
        }
    }

    pub fn mult(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.img * other.img,
            img: self.re * other.img + self.img * other.re,
        }
    }

    pub fn mag(self) -> f32 {
        f32::hypot(self.img, self.re)
    }

    pub fn scale(self, n: f32) -> Complex {
        Complex {
            re: self.re * n,
            img: self.img * n,
        }
    }
}

impl Lerp<f32> for Complex {
    fn lerp(self, other: Self, t: f32) -> Self {
        Complex {
            img: self.img.lerp(other.img, t),
            re: self.re.lerp(other.re, t),
        }
    }
}
