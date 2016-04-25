use std::collections::HashSet;
use pattern::{CustomPatternElem, Pattern, PatternElem};

/// A set of PatternElems, representing the set of valid operations at a given point in a sequence.
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
