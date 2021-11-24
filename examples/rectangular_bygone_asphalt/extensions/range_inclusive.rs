use super::f32::F32Extension;
use std::ops::RangeInclusive;

pub trait RangeInclusiveExtension {
    fn subdivide(&self, num_subdivisions: u16) -> Vec<f32>;
}

impl RangeInclusiveExtension for RangeInclusive<f32> {
    fn subdivide(&self, num_subdivisions: u16) -> Vec<f32> {
        let last_index = num_subdivisions - 1;
        (0..=last_index)
            .map(|index| {
                if last_index == 0 {
                    return 0.0;
                }

                let progress = index as f32 / last_index as f32;
                progress.denormalize(*self.start(), *self.end())
            })
            .collect()
    }
}
