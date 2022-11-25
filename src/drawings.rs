mod fourier_portrait;
mod treble_clef;

pub use fourier_portrait::FOURIER_PORTRAIT;
pub use treble_clef::TREBLE_CLEF;

use crate::complex::Complex;
pub const DRAWINGS: [Drawing; 2] = [
    Drawing {
        title: "Fourier",
        points: FOURIER_PORTRAIT,
    },
    Drawing {
        title: "TrebleClef",
        points: TREBLE_CLEF,
    },
];
pub struct Drawing {
    pub title: &'static str,
    pub points: &'static [Complex],
}
