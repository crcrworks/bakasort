pub trait BakaSort {
    fn sort<T>(v: Vec<T>) -> Vec<T>
    where
        T: PartialOrd + Ord;
}
