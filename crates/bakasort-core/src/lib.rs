pub trait BakaSort {
    type Item: PartialOrd + Ord;
    fn step<T, U>(array: T, sender: flume::Sender<Vec<Self::Item>>) -> U
    where
        T: IntoIterator<Item = Self::Item>,
        U: Iterator<Item = Self::Item>;
}
