use nannou::prelude::*;

pub trait Vector2Extension {
    fn between(a: &Vector2, b: &Vector2) -> Vector2;
    fn perpendicular_clockwise(&self) -> Vector2;
    fn perpendicular_counterclockwise(&self) -> Vector2;
}

impl Vector2Extension for Vector2 {
    fn between(a: &Vector2, b: &Vector2) -> Vector2 {
        *b - *a
    }
    fn perpendicular_clockwise(&self) -> Vector2 {
        vec2(self.y, -self.x)
    }
    fn perpendicular_counterclockwise(&self) -> Vector2 {
        vec2(-self.y, self.x)
    }
}
