use std::collections::HashSet;

use seq::{Seq, SeqElem};
use stepper::Stepper;

#[derive(Clone, Debug)]
pub struct SeqElemChoice(HashSet<SeqElem>);

impl SeqElemChoice {
    // TODO: Add ability to identify modulus.
    fn from_i32_pair(x: i32, y: i32) -> Self {
        let mut set = HashSet::new();
        set.insert(SeqElem::Const(y));
        set.insert(SeqElem::Plus(y - x));

        if x != 0 && y % x == 0 {
            set.insert(SeqElem::Mult(y / x));
        }

        if y != 0 && x % y == 0 {
            set.insert(SeqElem::Div(x / y));
        }

        SeqElemChoice(set)
    }
}

fn intersection(vec: &[SeqElemChoice]) -> HashSet<SeqElem> {
    let base = match vec.first() {
        Some(&SeqElemChoice(ref choices)) => choices.to_owned(),
        None => return HashSet::new()
    };

    vec.into_iter().fold(base, |set, choice| set.intersection(&choice.0).cloned().collect())
}

pub type Analyzer = Vec<SeqElemChoice>;

pub trait Analyze {
    fn from_slice(seq: &[i32]) -> Self;

    fn find_any_pattern_of_length(&self, n: usize) -> Option<Seq> {
        self.find_patterns_of_length(n).pop()
    }

    fn find_any_pattern(&self, max: usize) -> Option<Seq> {
        for i in 1..max {
            let mut vec = self.find_patterns_of_length(i);

            if !vec.is_empty() {
                return vec.pop();
            }
        }

        return None;
    }


    fn find_patterns_of_length(&self, n: usize) -> Vec<Seq>;

    fn find_patterns(&self, max: usize) -> Vec<Seq> {
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
        (0..seq.len() - 1).map(|i| SeqElemChoice::from_i32_pair(seq[i], seq[i + 1])).collect()
    }

    fn find_patterns_of_length(&self, range: usize) -> Vec<Seq> {
        let mut seqs = vec![Seq::empty()];

        for i in 0..range {
            let choices: Vec<_> = step!(i => self.len(); range).map(|i| self[i].to_owned()).collect();

            let mut new = Vec::new();

            for seq in seqs.iter_mut() {
                new.extend(seq.extend_each(intersection(&choices[..]).into_iter()));
            }

            seqs = new;
        }

        seqs
    }
}
