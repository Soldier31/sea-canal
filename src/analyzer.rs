use std::collections::HashSet;

use seq::{Seq, SeqElem};

#[derive(Debug)]
struct SeqElemChoice(HashSet<SeqElem>);

impl SeqElemChoice {
    // TODO: Add ability to identify modulus.
    fn from_i32_pair(x: i32, y: i32) -> SeqElemChoice {
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


fn get_choices(seq: &[i32]) -> Vec<SeqElemChoice> {
    (0..seq.len() - 1).map(|i| SeqElemChoice::from_i32_pair(seq[i], seq[i + 1])).collect::<Vec<_>>()
}

pub fn analyze_full(seq: &[i32]) -> Vec<SeqElem> {
    let choices = get_choices(seq);

    choices[0].0.iter().filter(|choice| choices.iter().all(|c| c.0.contains(choice))).cloned().collect::<Vec<_>>()
}
