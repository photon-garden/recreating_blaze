use nannou::prelude::*;

use super::rect::RectExtension;
use crate::snapshot::rand::Rand;

pub trait F32Extension {
    fn rescale(&self, input_min: f32, input_max: f32, output_min: f32, output_max: f32) -> f32;
    fn normalize(&self, input_min: f32, input_max: f32) -> f32;
    fn denormalize(&self, output_min: f32, output_max: f32) -> f32;
    fn denormalize_symmetrically(&self, radius: f32) -> f32;
    fn lerp_w(&self, rect: &Rect) -> f32;
    fn lerp_h(&self, rect: &Rect) -> f32;
    fn times(&self, other: f32) -> f32;
    fn plus(&self, other: f32) -> f32;
    fn divided_by(&self, other: f32) -> f32;
    fn clamp(&self, min: f32, max: f32) -> f32;
    fn normalize_w(&self, rect: &Rect) -> f32;
    fn normalize_h(&self, rect: &Rect) -> f32;
    fn turns(&self) -> f32;
    fn jitter(&self, rand: &mut Rand, amplitude: f32) -> f32;
    fn ease_in_quart(&self) -> f32;
    fn normalized_cos(&self) -> f32;
    fn normalized_sin(&self) -> f32;
    fn loop_normalize(&self, min: f32, max: f32) -> f32;
    fn close_to(&self, other: f32) -> bool;
}

impl F32Extension for f32 {
    fn rescale(&self, input_min: f32, input_max: f32, output_min: f32, output_max: f32) -> f32 {
        // Great explanation of this algorithm here:
        // https://stats.stackexchange.com/questions/281162/scale-a-number-between-a-range/281164
        let input_size = input_max - input_min;
        let normalized = (self - input_min) / input_size;

        let output_size = output_max - output_min;
        (normalized * output_size) + output_min
    }
    fn normalize(&self, input_min: f32, input_max: f32) -> f32 {
        self.rescale(input_min, input_max, 0.0, 1.0)
    }
    fn denormalize(&self, output_min: f32, output_max: f32) -> f32 {
        self.rescale(0.0, 1.0, output_min, output_max)
    }
    fn denormalize_symmetrically(&self, radius: f32) -> f32 {
        self.denormalize(-radius, radius)
    }
    fn lerp_w(&self, rect: &Rect) -> f32 {
        rect.lerp_w(*self)
    }
    fn lerp_h(&self, rect: &Rect) -> f32 {
        rect.lerp_h(*self)
    }
    fn times(&self, other: f32) -> f32 {
        self * other
    }
    fn plus(&self, other: f32) -> f32 {
        self + other
    }
    fn divided_by(&self, other: f32) -> f32 {
        self / other
    }
    fn clamp(&self, min: f32, max: f32) -> f32 {
        if self > &max {
            return max;
        }

        if self < &min {
            return min;
        }

        self.clone()
    }
    fn normalize_w(&self, rect: &Rect) -> f32 {
        rect.normalize_w(*self)
    }
    fn normalize_h(&self, rect: &Rect) -> f32 {
        rect.normalize_w(*self)
    }
    fn turns(&self) -> f32 {
        *self * TAU
    }
    fn jitter(&self, rand: &mut Rand, amplitude: f32) -> f32 {
        *self + rand.standard_gaussian() * amplitude
    }
    fn ease_in_quart(&self) -> f32 {
        nannou::ease::quart::ease_in(*self, 0.0, 1.0, 1.0)
    }
    fn normalized_cos(&self) -> f32 {
        self.cos().normalize(-1.0, 1.0)
    }
    fn normalized_sin(&self) -> f32 {
        self.sin().normalize(-1.0, 1.0)
    }
    // Takes a value that goes from min to max and normalizes it.
    // It handles out of bounds values by looping them back into
    // the normalized range. In contrast, regular normalization
    // just lets the value exceed the normalized range.
    //
    // For example:
    //
    // 20.0.normalize(0.0, 10.0); // Returns two.
    // 20.0.loop_normalize(0.0, 10.0) // Returns one.
    fn loop_normalize(&self, min: f32, max: f32) -> f32 {
        (self.normalize(min, max) % 1.0).abs()
    }
    fn close_to(&self, other: f32) -> bool {
        (self - other).abs() < f32::EPSILON
    }
}
