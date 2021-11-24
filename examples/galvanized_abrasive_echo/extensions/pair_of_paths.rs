use crate::prelude::*;

pub type PairOfPaths = Vec<(Point2, Point2)>;

pub trait PairOfPathsExtension {
    fn into_path(self) -> Path2;
}

impl PairOfPathsExtension for PairOfPaths {
    // If you have a polygon defined as left and right points
    // at each point along a path, like this:
    //
    // vec![
    //   (pt2(0.4, 0.4), pt2(0.6, 0.4)),
    //   (pt2(0.4, 0.6), pt2(0.6, 0.6)),
    // ]
    //
    // this method returns a flattened Vec that you can pass
    // to Nannou draw methods.
    //
    // vec![
    //   pt2(0.4, 0.4),
    //   pt2(0.4, 0.6),
    //   pt2(0.6, 0.4),
    //   pt2(0.6, 0.6)
    // ]
    fn into_path(self) -> Path2 {
        let (mut left_points, mut right_points): (Vec<_>, Vec<_>) = self.into_iter().unzip();

        right_points.reverse();
        left_points.append(&mut right_points);

        left_points
    }
}
