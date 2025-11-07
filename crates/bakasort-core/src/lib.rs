pub trait BakaSort {
    type Item: PartialOrd + Ord;
    fn step(v: Vec<Self::Item>) -> Vec<Self::Item>;
}

pub trait AsyncBakaSort {
    type Item: PartialOrd + Ord;
    fn step(v: Vec<Self::Item>) -> impl Future<Output = Vec<Self::Item>>;
}
