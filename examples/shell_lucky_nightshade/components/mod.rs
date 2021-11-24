use crate::prelude::*;

mod riley_circles;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        let container = params.container;
        let draw = params.draw;

        draw.background().color(soft_white());

        let circles = riley_circles::new();
        let stroke_weight = container.lerp_w(0.001);

        for circle in circles {
            let top_left = circle.top_left;
            let radius = container.lerp_w(circle.radius);
            let normalized_xy = pt2(top_left.x + circle.radius, top_left.y - circle.radius);
            let denormalized_xy = container.denormalize_xy(&normalized_xy);

            draw.ellipse()
                .xy(denormalized_xy)
                .radius(radius)
                .no_fill()
                .stroke_weight(stroke_weight)
                .stroke(soft_black());
        }
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}
