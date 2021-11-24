use crate::extensions::srgb8::*;
use nannou::prelude::*;

pub fn soft_white() -> Hsl {
    hsl(0.12466522, 0.34505126, 0.9302026)
}

pub fn soft_black() -> Hsl {
    srgb8(26, 26, 22).into_hsl()
}

pub fn riley_white() -> Hsl {
    srgb8(248, 244, 237).into_hsl()
}

pub fn riley_black() -> Hsl {
    srgb8(53, 47, 53).into_hsl()
}
