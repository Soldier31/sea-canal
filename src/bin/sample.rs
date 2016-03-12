extern crate sea_canal;

use sea_canal::{Analyze, Analyzer};

fn main() {
    let s = &[1, 2, 4, 5, 25];
    println!("{:?}", s);
    let analyzer = Analyzer::from_slice(s);

    for pat in analyzer.find_any_pattern(3) {
        println!("{}", pat);
    }

    let s = &[1, 4, 3, 6, 5];
    println!("{:?}", s);
    let analyzer = Analyzer::from_slice(s);

    for pat in analyzer.find_any_pattern(3) {
        println!("{}", pat);
    }

    let s = &[1, 2, 4, 8];
    println!("\n{:?}", s);
    let analyzer = Analyzer::from_slice(s);

    for pat in analyzer.find_any_pattern_of_length(1) {
        println!("{}", pat);
    }

    let s = &[1, 10, 19, 28];
    println!("\n{:?}", s);
    let analyzer = Analyzer::from_slice(s);

    for pat in analyzer.find_patterns(2) {
        println!("{}", pat);
    }

    let s = &[1, 9, 19, 28];
    println!("\n{:?}", s);
    let analyzer = Analyzer::from_slice(s);

    for pat in analyzer.find_patterns_of_length(1) {
        println!("{}", pat);
    }
}
