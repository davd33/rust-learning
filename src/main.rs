mod web_scrapper;
mod feature;
mod threads;
mod futures;

use crate::feature::Feature;

fn main() {
    let feature = Feature::FuturesMsgPassing;

	web_scrapper::get_faster_response(&feature);
	threads::count_to_10_threads(&feature);
	futures::count_to_10_tasks(&feature);
	futures::futures_msg_passing(&feature);
}
