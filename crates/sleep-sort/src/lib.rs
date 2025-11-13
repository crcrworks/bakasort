use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};

pub struct SleepSort {}

impl BakaSort for SleepSort {
    type Item = u64;

    fn sort(array: Vec<Self::Item>) -> Vec<Self::Item> {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let result = Arc::new(Mutex::new(Vec::<Self::Item>::new()));
            let mut handles = vec![];

            for a in array {
                let result_c = Arc::clone(&result);
                let handle = tokio::spawn(async move {
                    sleep(Duration::from_secs(a)).await;
                    result_c.lock().await.push(a);
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.await.unwrap();
            }

            let result = result.lock().await.clone();
            result
        })
    }
}
