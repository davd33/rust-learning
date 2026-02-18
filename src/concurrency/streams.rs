use trpl::StreamExt;
use crate::feature::Feature;
use crate::print::print_separator;

pub fn basic_stream(feature: &Feature) {
    if *feature != Feature::Streams {
        return;
    }

    println!("STREAM FROM VECTORS");

    trpl::block_on(async {
        let values = [1,2,3,4,5,6,7,8,9,10];
        let iter = values.iter().map(|x| x + 1);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(val) = stream.next().await {
            println!("{}", val);
        }
    });

    print_separator()
}
