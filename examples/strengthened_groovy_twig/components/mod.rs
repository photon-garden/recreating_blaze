use crate::prelude::*;

mod expansion_test;
mod riley_circles;
mod zigzags;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        let draw = params.draw;

        draw.background().color(soft_white());
        zigzags::render(params);
    }
}

fn render_circles() {
    // for circle in circles {
    //     let radius = container.lerp_w(circle.radius);
    //     let normalized_xy = circle.center();
    //     let denormalized_xy = container.denormalize_xy(&normalized_xy);

    //     draw.ellipse()
    //         .xy(denormalized_xy)
    //         .radius(radius)
    //         .no_fill()
    //         .stroke_weight(stroke_weight)
    //         .stroke(soft_black());
    // }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}
