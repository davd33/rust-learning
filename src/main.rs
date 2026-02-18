mod feature;
mod iterators;
mod print;
mod concurrency;

use concurrency::{futures, threads, web_scrapper};
use crate::concurrency::streams::basic_stream;
use crate::feature::Feature;
use crate::iterators::owned_iter;

fn main() {
    let feature = Feature::MovePinnedFuture;

	web_scrapper::get_faster_response(&feature);

	threads::count_to_10_threads(&feature);

	futures::count_to_10_tasks(&feature);
	futures::futures_msg_passing(&feature);
    futures::move_pinned_future(&feature);
    
    basic_stream(&feature);

    owned_iter(&feature);
}
