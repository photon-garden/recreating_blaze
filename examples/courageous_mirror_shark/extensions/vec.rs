pub trait VecExtension<T> {
    fn middle(&self) -> &T;
    fn looped_get(&self, index: usize, loop_size: usize) -> &T;
    fn normalized_get(&self, normalized_index: f32) -> &T;
    fn denormalize_index(&self, normalized_index: f32) -> usize;
    fn repeat_n(self, n: usize) -> itertools::RepeatN<Vec<T>>;
    fn normalized_enumerate(&self) -> Vec<(f32, &T)>;
    fn last_index(&self) -> usize;
}

impl<T> VecExtension<T> for Vec<T>
where
    T: std::clone::Clone,
{
    fn middle(&self) -> &T {
        let middle_index = self.len() / 2;
        self.get(middle_index).unwrap()
    }
    fn looped_get(&self, index: usize, loop_size: usize) -> &T {
        let looped_index = index % loop_size;
        self.get(looped_index).unwrap()
    }
    fn normalized_get(&self, normalized_index: f32) -> &T {
        let index = self.denormalize_index(normalized_index);
        self.get(index).unwrap()
    }
    fn denormalize_index(&self, normalized_index: f32) -> usize {
        (self.len() as f32 * normalized_index).floor() as usize
    }
    fn repeat_n(self, n: usize) -> itertools::RepeatN<Vec<T>> {
        itertools::repeat_n(self, n)
    }
    fn normalized_enumerate(&self) -> Vec<(f32, &T)> {
        let last_index = self.len() - 1;

        self.iter()
            .enumerate()
            .map(|(index, element)| {
                let normalized_index = index as f32 / last_index as f32;
                (normalized_index, element)
            })
            .collect()
    }
    fn last_index(&self) -> usize {
        self.len() - 1
    }
}
