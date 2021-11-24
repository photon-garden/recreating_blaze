use super::riley_circles;
use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let circles = riley_circles::new();
    let app = params.app;

    let zigzags = ZERO_TO_ONE
        .subdivide(72)
        .into_iter()
        .map(|center_angle| zigzag_polygon(app, center_angle, &circles));

    for zigzag in zigzags {
        let points = zigzag.denormalize_xy(params.container);
        params.draw.polygon().points(points).color(soft_black());
    }
}

fn zigzag_polygon(app: &App, center_angle: f32, circles: &[riley_circles::Circle]) -> Path2 {
    let offsets = vec![0.0, 0.04, 0.00, 0.04, 0.00, 0.04, 0.00, 0.04, 0.00];
    let width = app.normalized_mouse_x().denormalize(0.001, 0.012);
    dbg!(width);
    let widths = vec![
        width, width, width, width, width, width, width, width, width,
    ];

    let left_and_right_points = circles.iter().enumerate().map(|(index, circle)| {
        let width = widths.get(index).unwrap();
        let offset = offsets.get(index).unwrap();

        let angle = center_angle - offset;

        let left_point = circle.point_at(angle);
        let right_point = circle.point_at(angle + width);

        (left_point, right_point)
    });

    let (mut left_points, mut right_points): (Vec<_>, Vec<_>) =
        left_and_right_points.into_iter().unzip();

    right_points.reverse();
    left_points.append(&mut right_points);

    left_points
}
