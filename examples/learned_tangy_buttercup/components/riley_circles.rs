use crate::prelude::*;

pub struct Circle {
    pub top_left: Point2,
    pub radius: f32,
}

/*
9. Radius: 0.69093316. XY: 0.15552614, 0.14908854
8. Radius: 0.60357379. XY: 0.1932495, 0.17773438
7. Radius: 0.51819987. XY: 0.21641297, 0.21223958
6. Radius: 0.43017869. XY: 0.23891463, 0.26041667
5. Radius: 0.34348114. XY: 0.26869623, 0.32356771
4. Radius: 0.25943084. XY: 0.31568498, 0.38932292
3. Radius: 0.17140966. XY: 0.37855725, 0.4453125
2. Radius: 0.08603574. XY: 0.43878226, 0.47395833
1. Radius: 0.0443415. XY: 0.45863666, 0.49414062
*/
pub fn new() -> Vec<Circle> {
    vec![
        Circle {
            radius: 0.6909331 / 2.0,
            top_left: pt2(0.155526, 1.0 - 0.1490885),
        },
        Circle {
            radius: 0.6035737 / 2.0,
            top_left: pt2(0.193249, 1.0 - 0.1777343),
        },
        Circle {
            radius: 0.5181998 / 2.0,
            top_left: pt2(0.2164129, 1.0 - 0.2122395),
        },
        Circle {
            radius: 0.4301786 / 2.0,
            top_left: pt2(0.2389146, 1.0 - 0.2604166),
        },
        Circle {
            radius: 0.3434811 / 2.0,
            top_left: pt2(0.2686962, 1.0 - 0.3235677),
        },
        Circle {
            radius: 0.2594308 / 2.0,
            top_left: pt2(0.3156849, 1.0 - 0.3893229),
        },
        Circle {
            radius: 0.1714096 / 2.0,
            top_left: pt2(0.3785572, 1.0 - 0.445312),
        },
        Circle {
            radius: 0.0860357 / 2.0,
            top_left: pt2(0.4387822, 1.0 - 0.4739583),
        },
        Circle {
            radius: 0.044341 / 2.0,
            top_left: pt2(0.4586366, 1.0 - 0.4941406),
        },
    ]
}

impl Circle {
    pub fn center(&self) -> Point2 {
        let top_left = self.top_left;
        pt2(top_left.x + self.radius, top_left.y - self.radius)
    }

    pub fn point_at(&self, angle: NormalizedF32) -> Point2 {
        let turns = angle.turns();

        let x_offset = turns.cos().times(self.radius);
        let y_offset = turns.sin().times(self.radius);

        let offset = pt2(x_offset, y_offset);

        self.center() + offset
    }
}
