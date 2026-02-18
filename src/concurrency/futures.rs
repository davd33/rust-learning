use std::{sync::{Arc, Mutex}, time::Duration};
use std::pin::{pin, Pin};
use crate::feature::Feature;
use crate::print::print_separator;

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
    
    print_separator();
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

    print_separator()
}

pub fn move_pinned_future(feature: &Feature) {
    if *feature != Feature::MovePinnedFuture {
        return;
    }

    println!("MOVING A PINNED FUTURE");

    trpl::block_on(async {
        let print_smtg = pin!(async {
            println!("SMTG");
        });

        let print_smtg_else = pin!(async {
            trpl::sleep(Duration::from_secs(1)).await;
            println!("SMTH ELSE!");
        });

        let v: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![print_smtg, print_smtg_else];
        trpl::join_all(v).await;
    });

    print_separator()
}