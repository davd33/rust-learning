

fn main() {
    let  a: &str = "hello";
    let  b: &str = "world";
    let c: String = format!("{} {}", a, b);
    println!("{}", c);
}
