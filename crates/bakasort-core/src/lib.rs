pub trait BakaSort {
    type Item: PartialOrd + Ord;
    fn sort<T>(array: T, sender: flume::Sender<Vec<Self::Item>>) -> Vec<Self::Item>
    where
        T: IntoIterator<Item = Self::Item>;
}
