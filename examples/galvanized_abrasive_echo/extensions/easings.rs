use nannou::ease::*;
pub trait EasingsExtension {
    fn ease_out_circ(&self) -> f32;
    fn ease_in_circ(&self) -> f32;

    fn ease_in_cubic(&self) -> f32;
    fn ease_out_cubic(&self) -> f32;

    fn ease_in_quad(&self) -> f32;
    fn ease_out_quad(&self) -> f32;

    fn ease_in_sine(&self) -> f32;
    fn ease_out_sine(&self) -> f32;
}

impl EasingsExtension for f32 {
    fn ease_out_circ(&self) -> f32 {
        circ::ease_out(*self, 0.0, 1.0, 1.0)
    }
    fn ease_in_circ(&self) -> f32 {
        circ::ease_in(*self, 0.0, 1.0, 1.0)
    }
    fn ease_in_cubic(&self) -> f32 {
        cubic::ease_in(*self, 0.0, 1.0, 1.0)
    }
    fn ease_out_cubic(&self) -> f32 {
        cubic::ease_out(*self, 0.0, 1.0, 1.0)
    }
    fn ease_in_quad(&self) -> f32 {
        quad::ease_in(*self, 0.0, 1.0, 1.0)
    }
    fn ease_out_quad(&self) -> f32 {
        quad::ease_out(*self, 0.0, 1.0, 1.0)
    }

    fn ease_in_sine(&self) -> f32 {
        sine::ease_in(*self, 0.0, 1.0, 1.0)
    }
    fn ease_out_sine(&self) -> f32 {
        sine::ease_out(*self, 0.0, 1.0, 1.0)
    }
}
