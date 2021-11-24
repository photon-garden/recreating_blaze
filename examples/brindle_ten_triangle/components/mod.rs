use crate::prelude::*;

mod expansion_test;
mod riley_circles;
mod zigzag;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        expansion_test::render(params);

        // let container = params.container;
        // let draw = params.draw;

        // draw.background().color(soft_white());

        // let circles = riley_circles::new();
        // let stroke_weight = container.lerp_w(0.001);
        // let zigzags = zigzag::all(&circles);
        // for zigzag in zigzags {
        //     let points = zigzag.denormalize_xy(container);
        //     draw.polyline()
        //         .weight(stroke_weight)
        //         .points(points)
        //         .color(soft_black());
        // }
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
