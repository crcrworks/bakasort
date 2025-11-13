use rand::Rng;

pub struct BozoSort<T, R>
where
    T: PartialOrd + Ord + Clone,
    R: Rng,
{
    array: Vec<T>,
    rng: R,
}

impl<T, R> BozoSort<T, R>
where
    T: PartialOrd + Ord + Clone,
    R: Rng,
{
    fn new<I>(array: I, rng: R) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        BozoSort {
            array: array.into_iter().collect(),
            rng,
        }
    }

    fn array(&self) -> &Vec<T> {
        &self.array
    }
}

impl<T, R> Iterator for BozoSort<T, R>
where
    T: PartialOrd + Ord + Clone,
    R: Rng,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.array.is_sorted() {
            None
        } else {
            let randint1: usize = self.rng.random_range(0..self.array.len());
            let randint2: usize = self.rng.random_range(0..self.array.len());
            self.array.swap(randint1, randint2);
            Some(self.array.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    use super::*;
    #[test]
    fn three_vals() {
        let mut before = (0..11).collect::<Vec<i32>>();
        let after = before.clone();

        let mut rng = rand::rng();
        before.as_mut_slice().shuffle(&mut rng);

        let mut a = BozoSort::new(before, rng);
        while a.next().is_some() {}
        assert_eq!(*a.array(), after)
    }
}
