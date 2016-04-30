use std::collections::HashSet;
use std::iter::FromIterator;

use pattern::{CustomPatternElem, PatternElem};

/// A set of `PatternElems`, representing the set of valid operations at a given point in a sequence.
#[derive(Clone, Debug)]
pub struct PatternElemChoice(pub HashSet<PatternElem>);

impl PatternElemChoice {
    // TODO: Add ability to identify modulus.
    pub fn from_i32_pair(x: i32, y: i32, pats: Vec<CustomPatternElem>) -> Self {
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

impl IntoIterator for PatternElemChoice {
    type Item = PatternElem;
    type IntoIter = ::std::collections::hash_set::IntoIter<PatternElem>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<PatternElem> for PatternElemChoice {
    fn from_iter<I: IntoIterator<Item=PatternElem>>(iterator: I) -> Self {
        PatternElemChoice(iterator.into_iter().collect())
    }
}
