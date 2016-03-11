use std::collections::HashSet;

use seq::{Seq, SeqElem};

#[derive(Debug)]
struct SeqElemChoice(HashSet<SeqElem>);

impl SeqElemChoice {
    // TODO: Add ability to identify modulus.
    fn from_i32_pair(x: i32, y: i32) -> Self {
        let mut set = HashSet::new();
        set.insert(SeqElem::Const(y));
        set.insert(SeqElem::Plus(y - x));

        if x != 0 && y / x != 0 {
            set.insert(SeqElem::Mult(y / x));
        }

        if y != 0 && x / y != 0 {
            set.insert(SeqElem::Div(x / y));
        }

        SeqElemChoice(set)
    }
}


pub struct Analyzer(Vec<SeqElemChoice>);

impl Analyzer {
    pub fn from_seq(seq: &[i32]) -> Self {
        let vec = (0..seq.len() - 1).map(|i| SeqElemChoice::from_i32_pair(seq[i], seq[i + 1])).collect();

        Analyzer(vec)
    }

    pub fn analyze_one(&self) -> Vec<Seq> {
        self.0[0].0.iter().filter(|choice| self.0.iter().all(|c| c.0.contains(choice))).map(|choice| Seq::new(vec![choice.clone()])).collect::<Vec<_>>()
    }
}
