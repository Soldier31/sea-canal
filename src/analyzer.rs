use std::collections::HashSet;

use seq::{Seq, SeqElem};
use stepper::Stepper;

#[derive(Clone, Debug)]
struct SeqElemChoice(HashSet<SeqElem>);

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

pub type Analyzer = Vec<SeqElemChoice>;

pub trait Analyze {
    fn from_seq(seq: &[i32]) -> Self;
    fn analyze_n(&self, n: usize) -> Vec<Seq>;
}

fn intersection(vec: &[SeqElemChoice]) -> HashSet<SeqElem> {
    let base = match vec.first() {
        Some(&SeqElemChoice(ref choices)) => choices.to_owned(),
        None => return HashSet::new()
    };

    vec.into_iter().fold(base, |set, choice| set.intersection(&choice.0).cloned().collect())
}

impl Analyze for Analyzer {
    fn from_seq(seq: &[i32]) -> Self {
        (0..seq.len() - 1).map(|i| SeqElemChoice::from_i32_pair(seq[i], seq[i + 1])).collect()
    }

    fn analyze_n(&self, range: usize) -> Vec<Seq> {
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
