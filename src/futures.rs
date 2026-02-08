use std::{sync::{Arc, Mutex}, time::Duration};
use crate::feature::Feature;

pub fn count_to_10_tasks(feature: &Feature) {
    if *feature != Feature::Count10Tasks {
        return;
    }

    println!("COUNT TO 10 WITH TASKS");
    trpl::block_on(async {
        let cnt = Arc::new(Mutex::new(0));

        let mut futures = vec![];
        for i in 0..10 {
            let t1_cnt = Arc::clone(&cnt);
            let t1 = async move {
                println!("task {} waiting", i);
                trpl::sleep(Duration::from_millis((10-i)*100)).await;
                let mut n = t1_cnt.lock().unwrap();
                println!("task {}: {} + 1", i, *n);
                *n += 1;
            };
            futures.push(t1);
        }

        trpl::join_all(futures).await;

        println!("Last result = {}", cnt.lock().unwrap());
    });
    println!("---");
}

pub fn futures_msg_passing(feature: &Feature) {
    if *feature != Feature::FuturesMsgPassing {
        return;
    }

    println!("MESSAGE PASSING WITH FUTURES");

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let f1 = async move {
            let messages = vec![
                "hello bro",
                "how are you?",
            ];

            for msg in messages {
                tx.send(msg).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
        };

        let f2 = async {
            while let Some(received_msg) = rx.recv().await {
                println!("received = {}", received_msg);
            }
        };

        trpl::join(f1, f2).await;
    });

    println!("---");
}