use sea_canal::{Analyze, Analyzer};
use sea_canal::pattern::{CustomPatternElem, Pattern};
use sea_canal::pattern::PatternElem::*;

fn pow4(i: i32, j: i32) -> bool {
    i * i * i * i == j
}

fn root4(i: i32, j: i32) -> bool {
    j * j * j * j == i
}

#[test]
fn find_any_pattern() {
    let pow4_pattern = CustomPatternElem::new(pow4, "^4");
    let root4_pattern = CustomPatternElem::new(root4, "root 4");

    let slice = &[1, 2, 16, 2, 3, 81, 3];
    let analyzer = Analyzer::with_custom_patterns(slice, vec![pow4_pattern.clone(), root4_pattern.clone()]);

    assert_eq!(None, analyzer.find_any_pattern(1));
    assert_eq!(Some(pat![Plus(1), Custom(pow4_pattern), Custom(root4_pattern)]), analyzer.find_any_pattern(4));
}
