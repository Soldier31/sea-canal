extern crate sea_canal;

use std::io;

use sea_canal::{Analyze, Analyzer};

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("Unable to read input");
    let split = buf.split_whitespace();
    let nums : Vec<_> = split.map(|s| i32::from_str_radix(s, 10).expect("Invalid numeric input")).collect();
    let analyzer = Analyzer::from_slice(&nums);
    let length = nums.len();
    let x = length - 1;
    let y = length / 2 + 1;
    let n = if y < x { y } else { x };

    println!("----------");

    match analyzer.find_any_pattern(n) {
        Some(pat) => println!("{}", pat),
        None => println!("No pattern found")
    };
}
