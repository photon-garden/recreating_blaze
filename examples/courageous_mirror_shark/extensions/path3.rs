use nannou::prelude::*;

use super::RectExtension;

pub type Path3 = Vec<Point3<f32>>;

pub trait Path3Extension {
    fn denormalize_x_y(&self, rect: &Rect) -> Vec<Point2>;
    fn to_path2(&self) -> Vec<Point2>;
    fn polygon_contains(&self, point: &Point3) -> bool;
    fn find_closest(&self, point: &Point3) -> &Point3;
}

impl Path3Extension for Path3 {
    fn denormalize_x_y(&self, rect: &Rect) -> Vec<Point2> {
        self.iter()
            .map(|landmark| rect.denormalize_x_y(landmark.x, landmark.y))
            .collect()
    }
    fn to_path2(&self) -> Vec<Point2> {
        self.iter().map(|point| pt2(point.x, point.y)).collect()
    }
    fn polygon_contains(&self, point: &Point3) -> bool {
        let polygon = nannou::geom::Polygon::new(self.clone());
        let contains = polygon.contains(point);
        contains.is_some()
    }
    fn find_closest(&self, target: &Point3) -> &Point3 {
        let mut closest_so_far = self.first().unwrap();
        let mut smallest_distance_so_far = target.distance2(*closest_so_far);

        for point in self.iter() {
            let distance = target.distance2(*point);

            if distance < smallest_distance_so_far {
                closest_so_far = point;
                smallest_distance_so_far = distance;
            }
        }

        closest_so_far
    }
}
