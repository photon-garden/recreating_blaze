pub trait USizeExtension {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn floor_to_odd(&self) -> usize;
    fn is_divisible_by(&self, other_number: usize) -> bool;
}

impl USizeExtension for usize {
    fn is_divisible_by(&self, other_number: usize) -> bool {
        self % other_number == 0
    }
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
    fn is_odd(&self) -> bool {
        self % 2 != 0
    }
    fn floor_to_odd(&self) -> usize {
        if self.is_even() {
            self - 1
        } else {
            *self
        }
    }
}
