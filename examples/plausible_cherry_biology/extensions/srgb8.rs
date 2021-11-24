use nannou::color::{Hsl, IntoLinSrgba, Srgb};

pub trait Srgb8Extension {
    fn into_hsl(&self) -> Hsl;
}

impl Srgb8Extension for Srgb<u8> {
    fn into_hsl(&self) -> Hsl {
        self.into_lin_srgba().into()
    }
}
