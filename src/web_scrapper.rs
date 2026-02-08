use std::time::{Duration, Instant};
use trpl::Html;
use crate::feature::Feature;

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

pub fn get_faster_response(feature: &Feature) {
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