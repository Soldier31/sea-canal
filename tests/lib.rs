#[macro_use]
extern crate sea_canal;

use sea_canal::{Analyze, Analyzer};
use sea_canal::seq::Seq;
use sea_canal::seq::SeqElem::*;

#[test]
fn find_any_pattern_of_length() {
    let slice = &[1, 4, 3, 6, 5];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(None, analyzer.find_any_pattern_of_length(3));
    assert_eq!(Some(seq![Plus(3), Plus(-1)]), analyzer.find_any_pattern_of_length(2));
}

#[test]
fn find_any_pattern() {
    let slice = &[1, 4, 3, 6, 5];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(None, analyzer.find_any_pattern(1));
    assert_eq!(Some(seq![Plus(3), Plus(-1)]), analyzer.find_any_pattern(4));
}


#[test]
fn find_patterns_of_length() {
    let slice = &[2, 4, 2, 4, 2];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(Vec::<Seq>::new(), analyzer.find_patterns_of_length(3));
    assert_eq!(
        vec![
            seq![Const(4), Const(2)],
            seq![Const(4), Div(2)],
            seq![Const(4), Plus(-2)],
            seq![Const(4), SquareRoot],
            seq![Mult(2), Const(2)],
            seq![Mult(2), Div(2)],
            seq![Mult(2), Plus(-2)],
            seq![Mult(2), SquareRoot],
            seq![Plus(2), Const(2)],
            seq![Plus(2), Div(2)],
            seq![Plus(2), Plus(-2)],
            seq![Plus(2), SquareRoot],
            seq![Square, Const(2)],
            seq![Square, Div(2)],
            seq![Square, Plus(-2)],
            seq![Square, SquareRoot],
        ],
        analyzer.find_patterns_of_length(2)
    );
}

#[test]
fn find_patterns() {
    let slice = &[2, 4, 2, 4, 2];
    let analyzer = Analyzer::from_slice(slice);

    assert_eq!(Vec::<Seq>::new(), analyzer.find_patterns(1));
    assert_eq!(
        vec![
            seq![Const(4), Const(2)],
            seq![Const(4), Div(2)],
            seq![Const(4), Plus(-2)],
            seq![Const(4), SquareRoot],
            seq![Mult(2), Const(2)],
            seq![Mult(2), Div(2)],
            seq![Mult(2), Plus(-2)],
            seq![Mult(2), SquareRoot],
            seq![Plus(2), Const(2)],
            seq![Plus(2), Div(2)],
            seq![Plus(2), Plus(-2)],
            seq![Plus(2), SquareRoot],
            seq![Square, Const(2)],
            seq![Square, Div(2)],
            seq![Square, Plus(-2)],
            seq![Square, SquareRoot],
        ],
        analyzer.find_patterns(4)
    );
}