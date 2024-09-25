use std::env::args;

fn main() {
    let mapping = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    let mut out = String::new();
    let mut num = args().skip(1).next().unwrap().parse::<i32>().unwrap();
    for &(s, i) in mapping.iter() {
        while num >= i {
            out.push_str(s);
            num -= i;
        }
    }
    println!("{out}");
}
