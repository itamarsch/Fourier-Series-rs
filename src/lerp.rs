use std::ops::{Add, Mul};

impl<T> Lerp for T
where
    T: Add<Output = T> + Mul<f32, Output = T>,
{
    fn lerp(self, other: T, t: f32) -> T {
        self * (1.0 - t) + other * t
    }
}

pub trait Lerp {
    fn lerp(self, other: Self, t: f32) -> Self;
}
