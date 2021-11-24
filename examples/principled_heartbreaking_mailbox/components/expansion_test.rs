use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let stroke_weight = params.container.lerp_w(0.001);

    let path_len = 10;
    let mut magnitudes = Vec::with_capacity(path_len);
    let path: Vec<_> = ZERO_TO_ONE
        .subdivide(path_len as u16)
        .into_iter()
        .map(|progress: f32| {
            let x = progress.denormalize(0.2, 0.8);
            let y = progress.turns().normalized_sin().denormalize(0.4, 0.6);

            let normalized_magnitude = progress
                .times(2.0)
                .turns()
                .normalized_sin()
                .denormalize(0.001, 0.005);
            let magnitude = params.container.lerp_w(normalized_magnitude);
            magnitudes.push((magnitude, magnitude));

            params.container.denormalize_x_y(x, y)
        })
        .collect();

    let expanded_path = path.expand(&magnitudes);

    params
        .draw
        .polygon()
        .points(expanded_path)
        .color(soft_black());
}
