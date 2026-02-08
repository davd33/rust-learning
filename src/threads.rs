use std::sync::{Arc, Mutex};
use std::thread;
use crate::feature::Feature;

pub fn count_to_10_threads(feature: &Feature) {
    if *feature != Feature::Count10Threads {
        return;
    }

    println!("COUNT TO 10 WITH THREADS");

    let cnt = Arc::new(Mutex::new(0));

    let mut handlers = vec![];
    for i in 0..10 {
        let t1_cnt = Arc::clone(&cnt);
        let t1 = thread::spawn(move || {
            let mut n = t1_cnt.lock().unwrap();
            println!("thread {}: {} + 1", i,  *n);
            *n += 1;
        });
        handlers.push(t1);
    }

    for h in handlers {
        h.join().unwrap();
    }

    println!("Last result = {}", cnt.lock().unwrap());
    println!("---");
}