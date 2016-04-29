extern crate sea_canal;

use std::env;
use std::io;

use sea_canal::Analyzer;

fn main() {
    match env::args().nth(1) {
        Some(ref s) if s == "--sample"  => return sample(),
        _ => ()
    };

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("Unable to read input");
    let split = buf.split_whitespace();
    let nums : Vec<_> = split.map(|s| i32::from_str_radix(s, 10).expect("Invalid numeric input")).collect();
    let analyzer = Analyzer::with_meta(&nums);
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

fn sample() {
    let s = &[1, 2, 4, 5, 25];
    println!("Sequence: {:?}", s);
    let analyzer = Analyzer::from_slice(s);

    println!("Patterns:");
    for pat in analyzer.find_any_pattern(3) {
        println!("  {}", pat);
    }

    let s = &[2, 4, 2, 4];
    println!("\nSequence: {:?}", s);
    let analyzer = Analyzer::from_slice(s);

    for pat in analyzer.find_patterns_of_length(2) {
        println!("  {}", pat);
    }

    let s = &[1, 9, 19, 28];
    println!("\nSequence: {:?}", s);
    let analyzer = Analyzer::from_slice(s);

    println!("Patterns:");
    for pat in analyzer.find_any_pattern_of_length(1) {
        println!("  {}", pat);
    }
}
