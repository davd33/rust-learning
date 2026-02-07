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

fn get_faster_response(feature: &Feature) {
	if *feature != Feature::WebScrapper {
		return;
	}

	println!("GET FASTER RESPONSE");

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

	println!("---");
}

fn count_to_10_threads(feature: &Feature) {
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

fn count_to_10_tasks(feature: &Feature) {
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

fn futures_msg_passing(feature: &Feature) {
	if *feature != Feature::FuturesMsgPassing {
		return;
	}

	println!("MESSAGE PASSING WITH FUTURES");

	trpl::block_on(async {
		let (tx, mut rx) = trpl::channel();
		let messages = vec![
            "hello bro",
            "how are you?",
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            trpl::sleep(Duration::from_secs(1)).await;
        }

		while let Some(received_msg) = rx.recv().await {
            println!("received = {}", received_msg);
        }
	});

	println!("---");
}

#[derive(PartialEq, Eq)]
enum Feature {
    WebScrapper,
    Count10Threads,
    Count10Tasks,
    FuturesMsgPassing,
}

fn main() {
    let feature = Feature::FuturesMsgPassing;

	get_faster_response(&feature);
	count_to_10_threads(&feature);
	count_to_10_tasks(&feature);
	futures_msg_passing(&feature);
}
