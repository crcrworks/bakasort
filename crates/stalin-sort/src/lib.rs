use bakasort_core::BakaSort;

struct StalinSort {}

impl BakaSort for StalinSort {
    type Item = i32;

    fn sort<T>(array: T, sender: flume::Sender<Vec<Self::Item>>) -> Vec<Self::Item>
    where
        T: IntoIterator<Item = Self::Item>,
    {
        let mut result = Vec::new();
        let mut max = Self::Item::MIN;

        for i in array {
            if max < i {
                max = i;
                result.push(i);
            }

            let _ = sender.send(result.clone());
        }

        result
    }
}
