use crate::extensions::f32::*;
use nannou::noise::NoiseFn;
use nannou::noise::Perlin;
use nannou::noise::Seedable;
use nannou::noise::SuperSimplex;
use nannou::prelude::*;
use nanorand::RNG;

#[derive(Clone)]
pub struct Rand {
    seed: u64,
    rng: nanorand::WyRand,
    perlin_noise_generator: Option<Perlin>,
    super_simplex_noise_generator: Option<SuperSimplex>,
}

impl Rand {
    pub fn from_seed(seed: u64) -> Rand {
        let rng = nanorand::WyRand::new_seed(seed);
        Rand {
            seed,
            rng,
            perlin_noise_generator: None,
            super_simplex_noise_generator: None,
        }
    }

    pub fn zero_to_one(&mut self) -> f32 {
        let random_number = self.rng.generate::<u16>();
        (random_number as f32) / (u16::MAX as f32)
    }

    pub fn range(&mut self, lower: usize, upper: usize) -> usize {
        self.rng.generate_range(lower, upper)
    }

    // Ranges from -1.0 to 1.0.
    pub fn standard_gaussian(&mut self) -> f32 {
        let u1 = self.zero_to_one();
        let u2 = self.zero_to_one();

        let left = (-2.0 * u1.ln()).sqrt();
        let right = (TAU * u2).cos();

        left * right
    }

    pub fn clamped_standard_gaussian(&mut self) -> f32 {
        self.standard_gaussian().clamp(-1.0, 1.0)
    }

    pub fn normalized_gaussian(&mut self) -> f32 {
        self.clamped_standard_gaussian().normalize(-1.0, 1.0)
    }

    pub fn flip_coin(&mut self, probability: f32) -> bool {
        self.zero_to_one() < probability
    }

    pub fn pick_first<Choice>(
        &mut self,
        probability: f32,
        first: Choice,
        second: Choice,
    ) -> Choice {
        if self.flip_coin(probability) {
            first
        } else {
            second
        }
    }

    pub fn gaussian_point(&mut self) -> Point2 {
        let x = self.standard_gaussian();
        let y = self.standard_gaussian();

        pt2(x, y)
    }

    pub fn perlin_noise_generator(&mut self) -> Perlin {
        if self.perlin_noise_generator.is_none() {
            let perlin_noise_generator = Perlin::new();
            perlin_noise_generator.set_seed(self.seed as u32);
            self.perlin_noise_generator = Some(perlin_noise_generator);
        }

        self.perlin_noise_generator.unwrap()
    }

    // Output range is -1 to 1.
    // https://github.com/Razaekel/noise-rs/issues/149
    pub fn super_simplex_noise_generator(&mut self) -> SuperSimplex {
        if self.super_simplex_noise_generator.is_none() {
            let super_simplex_noise_generator = SuperSimplex::new().set_seed(self.seed as u32);
            self.super_simplex_noise_generator = Some(super_simplex_noise_generator);
        }

        self.super_simplex_noise_generator.unwrap()
    }

    pub fn curl(&mut self, point: &Point2, frequency: f32) -> Point2 {
        let x: f64 = point.x.times(frequency).into();
        let y: f64 = point.y.times(frequency).into();

        // We're going to approximate the derivative of
        // our noise function, so we need a very small
        // number to nudge our point by.
        let nudge_amount = 0.0001;
        let double_nudge_amount = nudge_amount * 2.0;

        let noise = self.super_simplex_noise_generator();

        let nudged_right = noise.get([x + nudge_amount, y]);
        let nudged_left = noise.get([x - nudge_amount, y]);

        // Approximate the derivative in the x direction.
        let dx = ((nudged_right - nudged_left) / double_nudge_amount) as f32;

        let nudged_up = noise.get([x, y + nudge_amount]);
        let nudged_down = noise.get([x, y - nudge_amount]);

        // Approximate the derivative in the y direction.
        let dy = ((nudged_up - nudged_down) / double_nudge_amount) as f32;

        // I ran an experiment where I input all combinations of x and y
        // ranging from -5,000 to 5,000. Then I measured the x and y outputs
        // of the noise function. I'm using those measurements to do this
        // normalization. The min and max for both x and y were the same.
        let output_x = dy.normalize(-3.6389112, 3.6389112);
        let output_y = dx.times(-1.0).normalize(-3.6389112, 3.6389112);

        pt2(output_x, output_y)
    }

    pub fn perlin_xy(&mut self, point: &Point2, frequency: f32) -> f32 {
        self.perlin_x_y(point.x, point.y, frequency)
    }

    pub fn perlin_x_y(&mut self, x: f32, y: f32, frequency: f32) -> f32 {
        let perlin_noise_generator = self.perlin_noise_generator();

        perlin_noise_generator.get([x.times(frequency).into(), y.times(frequency).into()]) as f32
    }

    pub fn super_simplex_x_y(&mut self, x: f32, y: f32, frequency: f32) -> f32 {
        let noise = self.super_simplex_noise_generator();

        let scaled_x = x * frequency;
        let scaled_y = y * frequency;

        let non_normalized_output = noise.get([scaled_x.into(), scaled_y.into()]) as f32;

        non_normalized_output.normalize(-1.0, 1.0)
    }

    pub fn super_simplex_xy(&mut self, point: &Point2, frequency: f32) -> f32 {
        self.super_simplex_x_y(point.x, point.y, frequency)
    }

    pub fn xy(&mut self) -> Point2 {
        pt2(self.zero_to_one(), self.zero_to_one())
    }

    pub fn element<'v, T>(&mut self, elements: &'v Vec<T>) -> &'v T {
        let index = self.index(elements);
        elements
            .get(index)
            .expect("Couldn't get a random element from array.")
    }

    pub fn index<T>(&mut self, elements: &Vec<T>) -> usize {
        let index: usize = self.rng.generate_range(0, elements.len());

        index
    }

    pub fn weighted_choice<Choice: Clone>(
        &mut self,
        choices_with_weights: &Vec<(Choice, f32)>,
    ) -> Choice {
        let weight_total: f32 = choices_with_weights
            .iter()
            .map(|(_choice, weight)| *weight)
            .sum();

        let mut remaining_distance = self.zero_to_one() * weight_total;

        for (choice, weight) in choices_with_weights {
            remaining_distance -= *weight;
            if remaining_distance <= 0.0 {
                return choice.clone();
            }
        }

        panic!("Couldn't pick a weighted choice.")
    }
}
