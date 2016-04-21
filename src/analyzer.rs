use std::collections::HashSet;

use pattern::{CustomPatternElem, Pattern, PatternElem};
use stepper::Stepper;

/// A set of PatternElems, representing the set of valid operations at a given point in a sequence.
#[derive(Clone, Debug)]
pub struct PatternElemChoice(HashSet<PatternElem>);

impl PatternElemChoice {
        // TODO: Add ability to identify modulus.
    fn from_i32_pair(x: i32, y: i32, pats: Vec<CustomPatternElem>) -> Self {
        let mut set = HashSet::new();
        set.insert(PatternElem::Const(y));
        set.insert(PatternElem::Plus(y - x));

        if x != 0 && y % x == 0 {
            set.insert(PatternElem::Mult(y / x));
        }

        if y != 0 && x % y == 0 {
            set.insert(PatternElem::Div(x / y));
        }

        if x * x == y {
            set.insert(PatternElem::Square);
        }

        if y * y == x {
            set.insert(PatternElem::SquareRoot);
        }

        if x * x * x == y {
            set.insert(PatternElem::Cube);
        }

        if y * y * y == x {
            set.insert(PatternElem::CubeRoot);
        }

        for custom in pats {
            if custom.check(x, y) {
                set.insert(PatternElem::Custom(custom));
            }
        }

        PatternElemChoice(set)
    }
}

fn intersection(slice: &[PatternElemChoice]) -> HashSet<PatternElem> {
    let base = match slice.first() {
        Some(&PatternElemChoice(ref choices)) => choices.to_owned(),
        None => return HashSet::new()
    };

    slice.into_iter().fold(base, |set, choice| set.intersection(&choice.0).cloned().collect())
}

/// Identifies patterns that describe a given sequence.
pub type Analyzer = Vec<PatternElemChoice>;

pub trait Analyze {
    /// Creates a new Analyze from a slice of integers.
    fn from_slice(seq: &[i32]) -> Self;

    /// Same as `from_slice`, but allows custom patterns to be specified.
    fn with_custom_patterns(seq: &[i32], pats: Vec<CustomPatternElem>) -> Self;

    /// Attempts to find exactly one pattern of `n` operations that described the given sequence.
    fn find_any_pattern_of_length(&self, n: usize) -> Option<Pattern> {
        self.find_patterns_of_length(n).pop()
    }

    /// Attempts to find exactly one pattern of maximum size `max` (in terms of number of
    /// operations) that describes the given sequence. It returns the smallest such pattern it can
    /// find .
    fn find_any_pattern(&self, max: usize) -> Option<Pattern> {
        for i in 1..max {
            let mut vec = self.find_patterns_of_length(i);

            if !vec.is_empty() {
                return vec.pop();
            }
        }

        return None;
    }

    /// Finds all patterns with `n` operations that describe the given sequence.
    fn find_patterns_of_length(&self, n: usize) -> Vec<Pattern>;

    /// Finds patterns of maximum size `max` (in terms of number of operations) that describe the
    /// given sequence. It will return all such patterns that are of minimal size (i.e. if a
    /// sequence can be described by a pattern of two operations, it will return all such patterns,
    /// but none of size three or greater).
    fn find_patterns(&self, max: usize) -> Vec<Pattern> {
        for i in 1..max {
            let vec = self.find_patterns_of_length(i);

            if !vec.is_empty() {
                return vec;
            }
        }

        return Vec::new();
    }
}

impl Analyze for Analyzer {
    fn from_slice(seq: &[i32]) -> Self {
        Self::with_custom_patterns(seq, Vec::new())
    }

    fn with_custom_patterns(seq: &[i32], pats: Vec<CustomPatternElem>) -> Self {
        (0..seq.len() - 1).map(|i|
            PatternElemChoice::from_i32_pair(seq[i], seq[i + 1], pats.clone())
        ).collect()
    }


    fn find_patterns_of_length(&self, range: usize) -> Vec<Pattern> {
        let mut pats = vec![Pattern::empty()];

        for i in 0..range {
            let choices: Vec<_> = step!(i => self.len(); range).map(|i| self[i].to_owned()).collect();

            let mut new = Vec::new();

            for pat in pats.iter_mut() {
                new.extend(pat.extend_each(intersection(&choices[..]).into_iter()));
            }

            pats = new;

            // Makes results deterministic, which is helpful.
            pats.sort();
        }

        pats
    }
}
