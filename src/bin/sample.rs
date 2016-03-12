extern crate sea_canal;

use sea_canal::analyzer::{Analyze, Analyzer};

fn main() {
    let s = &[1, 4, 3, 6, 5];
    println!("{:?}", s);
    let analyzer = Analyzer::from_seq(s);

    for seq in analyzer.analyze_n(2) {
        println!("{}", seq);
    }

    let s = &[1, 2, 4, 8];
    println!("\n{:?}", s);
    let analyzer = Analyzer::from_seq(s);

    for seq in analyzer.analyze_n(1) {
        println!("{}", seq);
    }

    let s = &[1, 10, 19, 28];
    println!("\n{:?}", s);
    let analyzer = Analyzer::from_seq(s);

    for seq in analyzer.analyze_n(1) {
        println!("{}", seq);
    }
}
