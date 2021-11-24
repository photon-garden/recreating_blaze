use crate::extensions::srgb8::*;
use nannou::prelude::*;

pub fn soft_white() -> Hsl {
    hsl(0.12466522, 0.34505126, 0.9302026)
}

pub fn soft_black() -> Hsl {
    srgb8(26, 26, 22).into_hsl()
}
