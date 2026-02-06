use std::{sync::{Arc, Mutex}, thread, time::{Duration, Instant}};

use trpl::Html;

async fn page_title(url: &str) -> (Option<String>, Duration) {
    let start = Instant::now();
    let resp_text = trpl::get(url).await.text().await;
    let duration = start.elapsed();
    
    (
	Html::parse(&resp_text)
            .select_first("title")
            .map(|title| title.inner_html()),
	duration
    )
}

fn get_faster_response() {
    trpl::block_on(async {
	println!("Fastest between Google and davd33.fr?");
	let title1 = page_title("https://davd33.fr/web/algos");
	let title2 = page_title("https://www.google.com/");

	let (title, dt) = match trpl::select(title1, title2).await {
	    trpl::Either::Left(l) => l,
	    trpl::Either::Right(r) => r,
	};
	
	match title {
	    Some(t) => {
		println!("Title = {} | dt = {:?}", t, dt);
	    },
	    None => println!("No title!")
	}
    });
}

fn count_to_10_threads() {
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
}

fn count_to_10_tasks() {
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
}

fn main() {
    println!("GET FASTER RESPONSE");
    get_faster_response();
    println!("---");
    println!("COUNT TO 10 WITH THREADS");
    count_to_10_threads();
    println!("---");
    println!("COUNT TO 10 WITH TASKS");
    count_to_10_tasks();
    println!("---");
}
