fn main() {
    let x = std::env::args()
        .skip(1)
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    println!("{x:b}");
}
