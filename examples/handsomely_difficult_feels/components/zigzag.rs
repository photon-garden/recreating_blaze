use super::riley_circles::*;
use crate::prelude::*;

pub fn all(circles: &[Circle]) -> Vec<Path2> {
    ZERO_TO_ONE
        .subdivide(72)
        .into_iter()
        .map(|center_angle| new(center_angle, circles))
        .collect()
}

fn new(center_angle: f32, circles: &[Circle]) -> Path2 {
    let offsets = vec![0.04, 0.0, 0.02, -0.01, 0.02, 0.0, 0.03, 0.02];

    let circles_and_offsets = circles.iter().zip(offsets.into_iter());

    circles_and_offsets
        .map(|(circle, offset)| {
            let angle = center_angle + offset;
            circle.point_at(angle)
        })
        .collect()
}
