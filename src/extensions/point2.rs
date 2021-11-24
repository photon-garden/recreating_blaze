use std::ops::RangeInclusive;

use crate::prelude::*;
use crate::snapshot::rand::Rand;
use nannou::prelude::*;

pub trait Point2Extension {
    fn reflect(&self, around: &Point2) -> Point2;
    fn lerp(&self, progress: f32, other: &Point2) -> Point2;
    fn jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Point2;
    fn jitter_mut(&mut self, rand: &mut Rand, x_jitter: f32, y_jitter: f32);
    fn gaussian_jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Point2;
    fn perlin_jitter_rotation(&self, rand: &mut Rand, distance: f32, frequency: f32) -> Point2;
    fn denormalize(&self, rect: &Rect) -> Point2;
    fn normalize_in_rect(&self, rect: &Rect) -> Point2;
    fn normalize_in_range(&self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Point2;
    fn outside_normalized_range(&self) -> bool;
    fn loop_normalize(&self, min: &Point2, max: &Point2) -> Point2;
    fn plus(&self, other: &Point2) -> Point2;
}

impl Point2Extension for Point2 {
    fn reflect(&self, around: &Point2) -> Point2 {
        let x_distance = self.x - around.x;
        let y_distance = self.y - around.y;

        let new_x = around.x - x_distance;
        let new_y = around.y - y_distance;

        pt2(new_x, new_y)
    }
    fn lerp(&self, progress: f32, other: &Point2) -> Point2 {
        let inverse_progress = 1.0 - progress;
        *self * inverse_progress + *other * progress
    }
    fn jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Point2 {
        let x_jitter = rand.zero_to_one() * x_jitter;
        let y_jitter = rand.zero_to_one() * y_jitter;

        Point2 {
            x: self.x + x_jitter,
            y: self.y + y_jitter,
        }
    }
    fn jitter_mut(&mut self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) {
        let x_jitter = rand.zero_to_one() * x_jitter;
        let y_jitter = rand.zero_to_one() * y_jitter;

        self.x += x_jitter;
        self.y += y_jitter;
    }
    fn gaussian_jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Point2 {
        let x_jitter = rand.standard_gaussian() * x_jitter;
        let y_jitter = rand.standard_gaussian() * y_jitter;

        Point2 {
            x: self.x + x_jitter,
            y: self.y + y_jitter,
        }
    }
    fn perlin_jitter_rotation(&self, rand: &mut Rand, distance: f32, frequency: f32) -> Point2 {
        let rotation = rand.perlin_xy(self, frequency);
        let offset = Vector2::from_angle(rotation.turns()) * distance;

        *self + offset
    }
    fn denormalize(&self, rect: &Rect) -> Point2 {
        rect.denormalize_xy(self)
    }
    fn outside_normalized_range(&self) -> bool {
        let normalized_range: RangeInclusive<f32> = 0.0..=1.0;

        !normalized_range.contains(&self.x) || !normalized_range.contains(&self.y)
    }
    fn normalize_in_rect(&self, rect: &Rect) -> Point2 {
        let normalized_x = self.x.normalize(rect.left(), rect.right());
        let normalized_y = self.y.normalize(rect.bottom(), rect.top());

        pt2(normalized_x, normalized_y)
    }
    fn normalize_in_range(&self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Point2 {
        let normalized_x = self.x.normalize(x_min, x_max);
        let normalized_y = self.y.normalize(y_min, y_max);

        pt2(normalized_x, normalized_y)
    }
    // Takes a point that goes from (0, 0) to (x_range, y_range)
    // and normalizes it. It handles out of bounds values by looping
    // them back into the normalized range. In contrast, regular
    // normalization just lets the value exceed the normalized range.
    fn loop_normalize(&self, min: &Point2, max: &Point2) -> Point2 {
        pt2(
            self.x.loop_normalize(min.x, max.x),
            self.y.loop_normalize(min.y, max.y),
        )
    }
    fn plus(&self, other: &Point2) -> Point2 {
        *self + *other
    }
}
