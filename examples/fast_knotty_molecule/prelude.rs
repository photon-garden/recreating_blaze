use std::ops::RangeInclusive;

pub use crate::colors::*;
pub use crate::components::*;
pub use crate::crates::*;
pub use crate::extensions::*;
pub use crate::grid::*;
pub use crate::snapshot::rand::Rand;
pub use crate::svg::*;

pub use nannou::prelude::*;

pub const ZERO_TO_ONE: RangeInclusive<f32> = 0.0..=1.0;
pub const PI_HALVES: f32 = PI / 2.0;

pub type NumberOfTurns = f32;
pub type NormalizedF32 = f32;

pub struct Model {
    pub snapshot: crate::snapshot::Snapshot,
    pub root_component: crate::components::Artwork,
    pub container: Rect<f32>,
}

pub struct RenderParams<'a> {
    pub app: &'a App,
    pub rand: &'a mut Rand,
    pub draw: &'a Draw,
    pub model: &'a Model,
    pub container: &'a Rect<f32>,
}

pub struct LineGroup {
    pub size: usize,
}

impl LineGroup {
    pub fn centered_indexes(&self) -> Vec<f32> {
        (0..self.size)
            .map(|index| self.center_index(index))
            .collect()
    }

    // If you have a length of 5, it converts indexes as follows:
    //
    // 0 becomes -2
    // 1 becomes -1
    // 2 becomes 0
    // 3 becomes 1
    // 4 becomes 2
    //
    // For an array with a length of 4:
    //
    // 0 becomes -1.5
    // 1 becomes -0.5
    // 2 becomes 0.5
    // 3 becomes 1.5
    //
    // Useful for computations where you have an array of elements
    // and you need to center them.
    pub fn center_index(&self, index: usize) -> f32 {
        let middle_index = if self.size.is_odd() {
            // If length is 5, we're iterating over indexes
            // from 0 to 4. The middle element of the array
            // will be at index 2.
            (self.size as f32 / 2.0).floor()
        } else {
            // If we're building an array with even length,
            // it doesn't actually have a middle element.
            // But if you think of it as having an *imaginary*
            // middle element halfway between the two most central
            // elements, the math works out.
            //
            // There's probably a more elegant way to think about this.
            (self.size as f32 / 2.0) - 0.5
        };

        index as f32 - middle_index
    }
}

#[derive(Copy, Clone)]
pub enum SineDirection {
    Ascending,
    Descending,
}

impl SineDirection {
    pub fn get(previous_value: f32, current_value: f32) -> Self {
        if previous_value < current_value {
            SineDirection::Ascending
        } else {
            SineDirection::Descending
        }
    }
}
