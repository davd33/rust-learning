use crate::feature::Feature;
use crate::print::print_separator;

struct A {
    a: u32
}

pub fn owned_iter(feature: &Feature) {
    if *feature != Feature::Iter {
        return;
    }

    println!("CREATE OWNED ITERATOR");

    let list1 = [A { a: 1 }, A { a: 2 }, A { a: 3 }];
    let mut list1_iter = list1.into_iter();

    println!("{}", list1_iter.next().unwrap().a);
    println!("{}", list1_iter.next().unwrap().a);

    let mut list1_iter2 = list1_iter.map(|x| A { a: x.a + 1 });
    println!("{}", list1_iter2.next().unwrap().a);

    // below does not compile
    // println!("{}", list1[0].a); // Value used after being moved [E0382]

    print_separator();
}