use super::riley_circles;
use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let circles = riley_circles::new();

    let zigzags = ZERO_TO_ONE
        .subdivide(72)
        .into_iter()
        .map(|center_angle| zigzag_polygon(center_angle, &circles));

    for zigzag in zigzags {
        let points = zigzag.denormalize_xy(params.container);
        params.draw.polygon().points(points).color(soft_black());
    }
}

fn zigzag_polygon(center_angle: f32, circles: &[riley_circles::Circle]) -> Path2 {
    let offsets = vec![0.04, 0.00, 0.02, -0.01, 0.02, 0.0, 0.03, 0.02];
    let magnitudes = vec![0.01, 0.008, 0.006, 0.004, 0.002, 0.001, 0.0005, 0.0002];

    let circles_and_offsets = circles.iter().zip(offsets.into_iter());

    let path: Vec<_> = circles_and_offsets
        .map(|(circle, offset)| {
            let angle = center_angle + offset;
            circle.point_at(angle)
        })
        .collect();

    path.expand_symmetrically(&magnitudes)
}
