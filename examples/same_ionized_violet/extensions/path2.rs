use std::ops::RangeInclusive;

use super::RectExtension;
use crate::prelude::*;

use super::Point2Extension;

pub type Path2 = Vec<Point2<f32>>;

type XRange = RangeInclusive<f32>;
type YRange = RangeInclusive<f32>;

pub trait Path2Extension {
    fn regular_polygon(center: &Point2, radius: f32, num_points: u16) -> Path2;
    fn smooth(&self, tightness: f32, repeats: u8) -> Path2;
    fn smooth_polygon(&self, tightness: f32, repeats: u8) -> Path2;
    fn jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Path2;
    fn jitter_mut(&mut self, rand: &mut Rand, x_jitter: f32, y_jitter: f32);
    fn gaussian_jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Path2;
    fn polygon_contains(&self, point: &Point2) -> bool;
    fn denormalize_xy(&self, rect: &Rect) -> Vec<Point2>;
    fn width_denormalize_xy(&self, rect: &Rect) -> Vec<Point2>;
    fn height_denormalize_xy(&self, rect: &Rect) -> Vec<Point2>;
    fn perlin_jitter_rotation(&self, rand: &mut Rand, distance: f32, frequency: f32) -> Path2;
    fn enblobben(&self, rand: &mut Rand, frequency: f32, amplitude: f32) -> Path2;
    fn find_closest(&self, target: &Point2) -> &Point2;
    fn normalize(&self) -> Vec<Point2>;
    fn x_y_range(&self) -> (f32, f32, f32, f32);
    fn invert_normalized_y(&self) -> Path2;
    fn into_geo_polygon(&self) -> geo::Polygon<f32>;
}

impl Path2Extension for Path2 {
    fn regular_polygon(center: &Point2, radius: f32, num_points: u16) -> Path2 {
        ZERO_TO_ONE
            .subdivide(num_points)
            .iter()
            .map(|angle| {
                let offset_x = angle.turns().cos().times(radius);
                let offset_y = angle.turns().sin().times(radius);

                pt2(center.x + offset_x, center.y + offset_y)
            })
            .collect()
    }
    // Chaikin's algorithm.
    fn smooth(&self, tightness: f32, repeats: u8) -> Path2 {
        smooth_line_or_polygon_repeatedly(self, tightness, repeats, false)
    }
    fn smooth_polygon(&self, tightness: f32, repeats: u8) -> Path2 {
        smooth_line_or_polygon_repeatedly(self, tightness, repeats, true)
    }
    fn jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Path2 {
        let jittery_path: Path2 = self
            .iter()
            .map(|point| point.jitter(rand, x_jitter, y_jitter))
            .collect();

        jittery_path
    }
    fn jitter_mut(&mut self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) {
        for point in self.iter_mut() {
            point.jitter_mut(rand, x_jitter, y_jitter);
        }
    }
    fn gaussian_jitter(&self, rand: &mut Rand, x_jitter: f32, y_jitter: f32) -> Path2 {
        self.iter()
            .map(|point| point.gaussian_jitter(rand, x_jitter, y_jitter))
            .collect()
    }
    fn perlin_jitter_rotation(&self, rand: &mut Rand, distance: f32, frequency: f32) -> Path2 {
        self.iter()
            .map(|point| point.perlin_jitter_rotation(rand, distance, frequency))
            .collect()
    }
    fn polygon_contains(&self, point: &Point2) -> bool {
        let polygon = nannou::geom::Polygon::new(self.clone());
        let contains = polygon.contains(point);
        contains.is_some()
    }
    fn denormalize_xy(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|point| rect.denormalize_xy(point))
            .collect()
    }
    fn width_denormalize_xy(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|point| rect.width_denormalize_xy(point))
            .collect()
    }
    fn height_denormalize_xy(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|point| rect.height_denormalize_xy(point))
            .collect()
    }
    fn normalize(&self) -> Vec<Point2> {
        let (x_min, x_max, y_min, y_max) = self.x_y_range();

        self.iter()
            .map(|point| point.normalize_in_range(x_min, x_max, y_min, y_max))
            .collect()
    }
    fn x_y_range(&self) -> (f32, f32, f32, f32) {
        let first = self.first().unwrap();

        let mut x_min = first.x;
        let mut y_min = first.y;

        let mut x_max = first.x;
        let mut y_max = first.y;

        for point in self.iter() {
            x_min = point.x.min(x_min);
            y_min = point.y.min(y_min);

            x_max = point.x.max(x_max);
            y_max = point.y.max(y_max);
        }

        (x_min, x_max, y_min, y_max)
    }
    fn enblobben(&self, rand: &mut Rand, frequency: f32, amplitude: f32) -> Path2 {
        let mut left_points = Vec::with_capacity(self.len());
        let mut right_points = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            let window_start;
            let window_end;
            let point_to_translate_from;

            if i == self.last_index() {
                window_start = self.get(i - 1).unwrap();
                window_end = self.get(i).unwrap();
                point_to_translate_from = window_end;
            } else {
                window_start = self.get(i).unwrap();
                window_end = self.get(i + 1).unwrap();
                point_to_translate_from = window_start;
            }

            let between = Vector2::between(window_start, window_end).normalize();

            let noise = rand.curl(point_to_translate_from, frequency);

            let left_magnitude = noise.x * amplitude;
            let right_magnitude = noise.y * amplitude;

            let perpendicular_counterclockwise =
                between.perpendicular_counterclockwise() * left_magnitude;
            let perpendicular_clockwise = between.perpendicular_clockwise() * right_magnitude;

            let left = *point_to_translate_from + perpendicular_counterclockwise;
            let right = *point_to_translate_from + perpendicular_clockwise;

            left_points.push(left);
            right_points.push(right);
        }

        right_points.reverse();
        left_points.append(&mut right_points);

        left_points
    }
    fn find_closest(&self, target: &Point2) -> &Point2 {
        let mut closest_so_far = self.first().unwrap();
        let mut smallest_distance_so_far = target.distance2(*closest_so_far);

        for point in self.iter() {
            let distance = target.distance2(*point);

            if distance < smallest_distance_so_far {
                closest_so_far = point;
                smallest_distance_so_far = distance;
            }
        }

        closest_so_far
    }
    fn invert_normalized_y(&self) -> Path2 {
        self.iter()
            .map(|point| pt2(point.x, 1.0 - point.y))
            .collect()
    }
    fn into_geo_polygon(&self) -> geo::Polygon<f32> {
        let line_string: geo::LineString<f32> = self
            .iter()
            .map(|point| (point.x, point.y))
            .collect::<Vec<(f32, f32)>>()
            .into();

        geo::Polygon::new(line_string, vec![])
    }
}

fn smooth_line_or_polygon_repeatedly(
    path: &Path2,
    tightness: f32,
    repeats: u8,
    is_polygon: bool,
) -> Path2 {
    if repeats == 0 {
        return path.to_owned();
    }

    let mut smoothed = smooth_line_or_polygon(path, tightness, is_polygon);

    for _ in 0..(repeats - 1) {
        smoothed = smooth_line_or_polygon(&smoothed, tightness, is_polygon);
    }

    smoothed
}

fn smooth_line_or_polygon(path: &Path2, tightness: f32, is_polygon: bool) -> Path2 {
    let num_line_segments = path.len() - 1;

    // Each line segment creates two points, then we add on the
    // first and last point from the original vector.
    let mut smoothed: Path2 = Vec::with_capacity(num_line_segments * 2 + 2);

    if !is_polygon {
        smoothed.push(*path.first().unwrap());
    }

    let inverse_tightness = 1.0 - tightness;

    for index in 0..path.last_index() {
        let point = path[index];
        let next_point = path[index + 1];

        let line_segment = [point, next_point];

        let near = line_segment.interpolate(tightness);
        let far = line_segment.interpolate(inverse_tightness);

        smoothed.push(near);
        smoothed.push(far);
    }

    if !is_polygon {
        smoothed.push(*path.last().unwrap());
    }

    if is_polygon {
        let first = *path.first().unwrap();
        let mut last = *path.last().unwrap();

        if first == last {
            let next_to_last_index = path.last_index() - 1;
            last = *path.get(next_to_last_index).unwrap();
        }

        let point = last;
        let next_point = first;
        let line_segment = [point, next_point];

        let near = line_segment.interpolate(tightness);
        let far = line_segment.interpolate(inverse_tightness);

        smoothed.push(near);
        smoothed.push(far);
    }

    smoothed
}
