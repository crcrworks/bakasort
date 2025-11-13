use rand::{Rng, seq::SliceRandom};

pub struct BogoSort<T, R>
where
    T: PartialOrd + Ord + Clone,
    R: Rng,
{
    array: Vec<T>,
    rng: R,
}

impl<T, R> Iterator for BogoSort<T, R>
where
    T: PartialOrd + Ord + Clone,
    R: Rng,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.array.is_sorted() {
            None
        } else {
            self.array.as_mut_slice().shuffle(&mut self.rng);
            Some(self.array.clone())
        }
    }
}
