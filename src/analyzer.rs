use std::collections::HashSet;

use choice::PatternElemChoice;
use pattern::{CustomPatternElem, Pattern, PatternElem};
use stepper::Stepper;

/// Identifies patterns that describe a given sequence.
pub struct Analyzer {
    choices: Vec<PatternElemChoice>,
    meta_count: usize,
}

impl Analyzer {
    /// Creates a new Analyze from a slice of integers.
    pub fn from_slice(seq: &[i32]) -> Self {
        Self::with_custom_patterns(seq, Vec::new())
    }

    /// Same as `from_slice`, but allows custom patterns to be specified.
    pub fn with_custom_patterns(seq: &[i32], pats: Vec<CustomPatternElem>) -> Self {
        Analyzer {
            meta_count: 0,
            choices: (0..seq.len() - 1).map(|i|
                         PatternElemChoice::from_i32_pair(seq[i], seq[i + 1], pats.clone())
                     ).collect()
        }
    }

    /// Attempts to find exactly one pattern of `n` operations that described the given sequence.
    pub fn find_any_pattern_of_length(&self, n: usize) -> Option<Pattern> {
        // TODO: Short-circuit finding one pattern instead of all of them
        self.find_patterns_of_length(n).pop()
    }

    /// Attempts to find exactly one pattern of maximum size `max` (in terms of number of
    /// operations) that describes the given sequence. It returns the smallest such pattern it can
    /// find .
    pub fn find_any_pattern(&self, max: usize) -> Option<Pattern> {
        for i in 1..max {
            let mut vec = self.find_patterns_of_length(i);

            // TODO: Short-circuit finding one pattern instead of all of them
            if !vec.is_empty() {
                return vec.pop();
            }
        }

        return None;
    }

    /// Finds all patterns with `n` operations that describe the given sequence.
    pub fn find_patterns_of_length(&self, range: usize) -> Vec<Pattern> {
        let mut pats = vec![Pattern::empty()];

        for i in 0..range {
            let choices: Vec<_> = step!(i => self.len(); range).map(|i| self.choices[i].clone()).collect();

            let mut new = Vec::new();

            for pat in pats.iter_mut() {
                new.extend(pat.extend_each(Self::intersection(&choices[..]).into_iter()));
            }

            pats = new;

            // Makes results deterministic, which is helpful.
            pats.sort();
        }

        pats
    }

    /// Finds patterns of maximum size `max` (in terms of number of operations) that describe the
    /// given sequence. It will return all such patterns that are of minimal size (i.e. if a
    /// sequence can be described by a pattern of two operations, it will return all such patterns,
    /// but none of size three or greater).
    pub fn find_patterns(&self, max: usize) -> Vec<Pattern> {
        for i in 1..max {
            let vec = self.find_patterns_of_length(i);

            if !vec.is_empty() {
                return vec;
            }
        }

        return Vec::new();
    }

    #[inline]
    fn len(&self) -> usize {
        self.choices.len()
    }

    fn intersection(slice: &[PatternElemChoice]) -> HashSet<PatternElem> {
        let base = match slice.first() {
            Some(&PatternElemChoice(ref choices)) => choices.clone(),
            None => return HashSet::new()
        };

        slice.into_iter().fold(base, |set, choice| set.intersection(&choice.0).cloned().collect())
    }
}
