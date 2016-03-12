#[macro_use]
extern crate sea_canal;

use sea_canal::{Analyze, Analyzer};
use sea_canal::pattern::Pattern;
use sea_canal::pattern::PatternElem::*;

#[test]
fn find_any_pattern_of_length() {
    let slice = &[1, 4, 3, 6, 5];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(None, analyzer.find_any_pattern_of_length(3));
    assert_eq!(Some(pat![Plus(3), Plus(-1)]), analyzer.find_any_pattern_of_length(2));
}

#[test]
fn find_any_pattern() {
    let slice = &[1, 4, 3, 6, 5];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(None, analyzer.find_any_pattern(1));
    assert_eq!(Some(pat![Plus(3), Plus(-1)]), analyzer.find_any_pattern(4));
}


#[test]
fn find_patterns_of_length() {
    let slice = &[2, 4, 2, 4, 2];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(Vec::<Pattern>::new(), analyzer.find_patterns_of_length(3));
    assert_eq!(
        vec![
            pat![Const(4), Const(2)],
            pat![Const(4), Div(2)],
            pat![Const(4), Plus(-2)],
            pat![Const(4), SquareRoot],
            pat![Mult(2), Const(2)],
            pat![Mult(2), Div(2)],
            pat![Mult(2), Plus(-2)],
            pat![Mult(2), SquareRoot],
            pat![Plus(2), Const(2)],
            pat![Plus(2), Div(2)],
            pat![Plus(2), Plus(-2)],
            pat![Plus(2), SquareRoot],
            pat![Square, Const(2)],
            pat![Square, Div(2)],
            pat![Square, Plus(-2)],
            pat![Square, SquareRoot],
        ],
        analyzer.find_patterns_of_length(2)
    );
}

#[test]
fn find_patterns() {
    let slice = &[2, 4, 2, 4, 2];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(Vec::<Pattern>::new(), analyzer.find_patterns(1));
    assert_eq!(
        vec![
            pat![Const(4), Const(2)],
            pat![Const(4), Div(2)],
            pat![Const(4), Plus(-2)],
            pat![Const(4), SquareRoot],
            pat![Mult(2), Const(2)],
            pat![Mult(2), Div(2)],
            pat![Mult(2), Plus(-2)],
            pat![Mult(2), SquareRoot],
            pat![Plus(2), Const(2)],
            pat![Plus(2), Div(2)],
            pat![Plus(2), Plus(-2)],
            pat![Plus(2), SquareRoot],
            pat![Square, Const(2)],
            pat![Square, Div(2)],
            pat![Square, Plus(-2)],
            pat![Square, SquareRoot],
        ],
        analyzer.find_patterns(4)
    );
}
