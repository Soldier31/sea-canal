use analyzer::Analyzer;
use choice::PatternElemChoice;
use pattern::Pattern;

#[derive(Debug)]
pub struct MetaAnalyzer {
    choices: Vec<PatternElemChoice>,
}

impl MetaAnalyzer {
    pub fn new(choices: Vec<PatternElemChoice>) -> Self {
        MetaAnalyzer { choices: choices }
    }

    // TODO: implement in a non-terrible way
    pub fn find_patterns(&self) -> Vec<Pattern> {
        let mut patterns = vec![Pattern::empty()];

        for choice in &self.choices {
            let mut new = Vec::new();

            for pat in &mut patterns {
                new.extend(pat.extend_each(choice.clone().into_iter()));
            }

            patterns = new;
        }

        patterns.into_iter().filter(|pat| {
            if pat.is_empty() {
                return false;
            }

            // I hate having to do a loop instead of using iterators, but there's no way to return
            // from the outer function from a closure. I hate myself for saying this, but too bad
            // Rust doesn't have monads...
            let mut operands = Vec::new();

            for elem in pat.iter() {
                match elem.get_operand() {
                    Some(op) => operands.push(op),
                    None => return false,
                }
            }

            let analyzer = Analyzer::from_slice(&operands);
            if analyzer.find_any_pattern_of_length(1).is_none() {
                return false;
            }

            pat.has_repeating_types()
        }).collect()
    }
}
