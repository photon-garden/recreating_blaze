use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let stroke_weight = params.container.lerp_w(0.001);

    let path = ZERO_TO_ONE.subdivide(10).into_iter().map(|progress: f32| {
        let x = progress.denormalize(0.2, 0.8);
        let y = progress.turns().normalized_sin().denormalize(0.4, 0.6);

        params.container.denormalize_x_y(x, y)
    });

    params
        .draw
        .polyline()
        .weight(stroke_weight)
        .points(path)
        .color(soft_black());
}
