use nannou::color::Hsl;

use super::rbg_hue::RgbHueExtension;

pub trait HslExtension {
    fn to_code(&self) -> String;
}

impl HslExtension for Hsl {
    fn to_code(&self) -> String {
        let hue = self.hue.to_normalized();
        format!(
            "let color = hsl({}, {}, {});",
            hue, self.saturation, self.lightness
        )
    }
}
