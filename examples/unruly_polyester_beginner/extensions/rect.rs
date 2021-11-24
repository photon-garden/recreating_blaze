use std::ops::RangeInclusive;

use super::f32::F32Extension;
use crate::snapshot::rand::Rand;
use nannou::prelude::*;

use crate::grid::Grid;

pub trait RectExtension {
    fn unit() -> Rect;
    fn minus_one_to_one() -> Rect;
    fn left_right(&self) -> RangeInclusive<f32>;
    fn bottom_top(&self) -> RangeInclusive<f32>;
    fn normalized_pad_w(&self, padding: f32) -> Rect;
    fn pad_w(&self, padding: f32) -> Rect;
    fn normalized_pad_h(&self, padding: f32) -> Rect;
    fn pad_h(&self, padding: f32) -> Rect;
    fn scale(&self, w_amount: f32, h_amount: f32) -> Rect;
    fn scale_w(&self, amount: f32) -> Rect;
    fn scale_h(&self, amount: f32) -> Rect;
    fn scale_wh(&self, amount: f32) -> Rect;
    fn lerp_w(&self, x: f32) -> f32;
    fn lerp_h(&self, x: f32) -> f32;
    fn lerp_wh(&self, point: &Vector2<f32>) -> Vector2<f32>;
    fn denormalize_x(&self, x: f32) -> f32;
    fn denormalize_y(&self, y: f32) -> f32;
    fn denormalize_xy(&self, point: &Point2) -> Point2;
    fn denormalize_x_y(&self, x: f32, y: f32) -> Point2;
    fn width_denormalize_xy(&self, point: &Point2) -> Point2;
    fn height_denormalize_xy(&self, point: &Point2) -> Point2;
    fn width_denormalize_x_y(&self, x: f32, y: f32) -> Point2;
    fn height_denormalize_x_y(&self, x: f32, y: f32) -> Point2;
    fn normalize_w(&self, x: f32) -> f32;
    fn normalize_h(&self, y: f32) -> f32;
    fn normalize_wh(&self, point: &Point2) -> Point2;
    fn grid(&self, num_columns: usize, num_rows: usize) -> Grid;
    fn child(
        &self,
        top_left_x: f32,
        top_left_y: f32,
        bottom_right_x: f32,
        bottom_right_y: f32,
    ) -> Rect;
    fn point_within(&self, x: f32, y: f32) -> Point2;
    fn random_point_within(&self, rand: &mut Rand) -> Point2;
}

impl RectExtension for Rect {
    fn unit() -> Rect {
        Rect::from_corners(pt2(0.0, 0.0), pt2(1.0, 1.0))
    }
    fn minus_one_to_one() -> Rect {
        Rect::from_corners(pt2(-1.0, -1.0), pt2(1.0, 1.0))
    }
    fn left_right(&self) -> RangeInclusive<f32> {
        self.left()..=self.right()
    }
    fn bottom_top(&self) -> RangeInclusive<f32> {
        self.bottom()..=self.top()
    }
    fn normalized_pad_w(&self, normalized_padding: f32) -> Rect {
        let padding = self.lerp_w(normalized_padding);
        self.pad_w(padding)
    }
    fn pad_w(&self, padding: f32) -> Rect {
        self.pad_left(padding).pad_right(padding)
    }
    fn normalized_pad_h(&self, normalized_padding: f32) -> Rect {
        let padding = self.lerp_h(normalized_padding);
        self.pad_h(padding)
    }
    fn pad_h(&self, padding: f32) -> Rect {
        self.pad_top(padding).pad_bottom(padding)
    }
    fn scale(&self, w_amount: f32, h_amount: f32) -> Rect {
        let new_w = self.w() * w_amount;
        let new_h = self.h() * h_amount;
        Rect::from_x_y_w_h(self.x(), self.y(), new_w, new_h)
    }
    fn scale_w(&self, amount: f32) -> Rect {
        let new_w = self.w() * amount;
        Rect::from_x_y_w_h(self.x(), self.y(), new_w, self.h())
    }
    fn scale_h(&self, amount: f32) -> Rect {
        let new_h = self.h() * amount;
        Rect::from_x_y_w_h(self.x(), self.y(), self.w(), new_h)
    }
    fn scale_wh(&self, amount: f32) -> Rect {
        self.scale(amount, amount)
    }
    fn lerp_w(&self, x: f32) -> f32 {
        x * self.w()
    }
    fn lerp_h(&self, y: f32) -> f32 {
        y * self.h()
    }
    fn lerp_wh(&self, point: &Vector2<f32>) -> Vector2<f32> {
        pt2(self.lerp_w(point.x), self.lerp_h(point.y))
    }
    fn denormalize_x(&self, x: f32) -> f32 {
        self.left() + self.lerp_w(x)
    }
    fn denormalize_y(&self, y: f32) -> f32 {
        self.bottom() + self.lerp_h(y)
    }
    fn denormalize_xy(&self, point: &Point2) -> Point2 {
        self.denormalize_x_y(point.x, point.y)
    }
    fn denormalize_x_y(&self, x: f32, y: f32) -> Point2 {
        Point2 {
            x: self.denormalize_x(x),
            y: self.denormalize_y(y),
        }
    }
    fn width_denormalize_xy(&self, point: &Point2) -> Point2 {
        self.width_denormalize_x_y(point.x, point.y)
    }
    fn height_denormalize_xy(&self, point: &Point2) -> Point2 {
        self.height_denormalize_x_y(point.x, point.y)
    }
    fn width_denormalize_x_y(&self, x: f32, y: f32) -> Point2 {
        Point2 {
            x: self.denormalize_x(x),
            y: self.denormalize_x(y),
        }
    }
    fn height_denormalize_x_y(&self, x: f32, y: f32) -> Point2 {
        Point2 {
            x: self.denormalize_y(x),
            y: self.denormalize_y(y),
        }
    }
    fn normalize_w(&self, x: f32) -> f32 {
        x.normalize(self.left(), self.right())
    }
    fn normalize_h(&self, y: f32) -> f32 {
        y.normalize(self.bottom(), self.top())
    }
    fn normalize_wh(&self, point: &Point2) -> Point2 {
        Point2 {
            x: self.normalize_w(point.x),
            y: self.normalize_h(point.y),
        }
    }
    fn grid(&self, num_columns: usize, num_rows: usize) -> Grid {
        Grid::new(self.clone(), num_columns, num_rows)
    }

    fn child(
        &self,
        top_left_x: f32,
        top_left_y: f32,
        bottom_right_x: f32,
        bottom_right_y: f32,
    ) -> Rect {
        let top_left = self.point_within(top_left_x, top_left_y);
        let bottom_right = self.point_within(bottom_right_x, bottom_right_y);

        Rect::from_corners(top_left, bottom_right)
    }
    fn point_within(&self, x: f32, y: f32) -> Point2 {
        let new_x = x.denormalize(self.left(), self.right());
        let new_y = y.denormalize(self.bottom(), self.top());

        pt2(new_x, new_y)
    }
    fn random_point_within(&self, rand: &mut Rand) -> Point2 {
        let x = rand.zero_to_one();
        let y = rand.zero_to_one();

        self.point_within(x, y)
    }
}
