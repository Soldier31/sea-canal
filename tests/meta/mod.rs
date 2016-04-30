use sea_canal::Analyzer;
use sea_canal::Pattern;
use sea_canal::PatternElem::*;

#[test]
fn meta_find_any_pattern() {
    let slice = &[1, 2, 4, 7, 11];
    let analyzer = Analyzer::with_meta(slice);

    assert_eq!(Some(pat!(Meta(pat!(Plus(1), Plus(2), Plus(3), Plus(4))))), analyzer.find_any_pattern(1));
}

#[test]
fn meta_find_patterns_of_length() {
    let slice = &[10, 11, 10, 12, 10, 13, 10];
    let analyzer = Analyzer::with_meta(slice);

    assert_eq!(Vec::<Pattern>::new(), analyzer.find_patterns_of_length(1));
    assert_eq!(
        vec![
            pat!(Meta(pat!(Plus(1), Plus(2), Plus(3))), Const(10)),
            pat!(Meta(pat!(Plus(1), Plus(2), Plus(3))), Meta(pat!(Plus(-1), Plus(-2), Plus(-3))))
        ],
        analyzer.find_patterns_of_length(2));
}

#[test]
fn mixed_operand_meta_pattern() {
    let slice = &[10, 11, 10, 20, 10, 13, 10, 40];
    let analyzer = Analyzer::with_meta(slice);

    assert_eq!(None, analyzer.find_any_pattern(1));
    assert_eq!(Some(pat!(Meta(pat!(Plus(1), Mult(2), Plus(3), Mult(4))), Const(10))), analyzer.find_any_pattern(4));
}
