pub trait BakaSort<I>
where
    I: PartialOrd + Ord,
{
    fn sort<T>(array: T, sender: flume::Sender<Vec<I>>) -> Vec<I>
    where
        T: IntoIterator<Item = I>;
}
