pub trait IteratorExtension<I, T> {
    fn repeat_n(self, n: usize) -> itertools::RepeatN<I>;
}

impl<I, T> IteratorExtension<I, T> for I
where
    I: Iterator<Item = T> + Clone,
{
    fn repeat_n(self, n: usize) -> itertools::RepeatN<I> {
        itertools::repeat_n(self, n)
    }
}
