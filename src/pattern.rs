use std::fmt::{Display, Formatter, Error};

/// Operations from one integer to another.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PatternElem {
    // Listed alphabetically to make equality sorting intuitive.
    Const(i32),
    Cube,
    CubeRoot,
    Custom(CustomPatternElem),
    Div(i32),
    Meta(Box<PatternElem>, usize),
    Mod(i32),
    Mult(i32),
    Plus(i32),
    Square,
    SquareRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomPatternElem {
    check: fn(i32, i32) -> bool,
    repr: String,
}

impl CustomPatternElem {
    pub fn new(check: fn(i32, i32) -> bool, repr: &str) -> Self {
        CustomPatternElem { check: check, repr: String::from(repr) }
    }

    pub fn check(&self, x: i32, y: i32) -> bool {
        let check = self.check;
        check(x, y)
    }
}

impl Display for PatternElem {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            PatternElem::Const(i) => write!(fmt, "={}", i),
            PatternElem::Plus(i) if i < 0 => write!(fmt, "-{}", i.abs()),
            PatternElem::Plus(i) => write!(fmt, "+{}", i),
            PatternElem::Mult(i) => write!(fmt, "*{}", i),
            PatternElem::Div(i) => write!(fmt, "/{}", i),
            PatternElem::Mod(i) => write!(fmt, "%{}", i),
            PatternElem::Square => write!(fmt, "^2"),
            PatternElem::Cube => write!(fmt, "^3"),
            PatternElem::SquareRoot => write!(fmt, "root 2"),
            PatternElem::CubeRoot => write!(fmt, "root 3"),
            PatternElem::Custom(CustomPatternElem { ref repr, .. }) => write!(fmt, "{}", repr),
            PatternElem::Meta(ref elem, id) => write!(fmt, "{}[{}]", elem, id),
        }
    }
}

/// A sequence of operations defining a pattern.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pattern(Vec<PatternElem>);

#[macro_export]
macro_rules! pat {
    ($($elem:expr),*) => (Pattern::new(vec![$($elem),*]))
}

impl Pattern {
    /// Constructs a new pattern given a vector of operations.
    pub fn new(elems: Vec<PatternElem>) -> Self {
        Pattern(elems)
    }

    /// Constructs a new empty pattern.
    pub fn empty() -> Self {
        Pattern::new(Vec::new())
    }

    /// Appends each of the items in `iter` to the pattern separately, returning a vector of
    /// patterns.
    ///
    /// ```
    /// # #[macro_use] extern crate sea_canal;
    /// # use sea_canal::pattern::Pattern;
    /// # use sea_canal::pattern::PatternElem::{Plus, Mult, Div};
    /// # fn main() {
    /// let pat = pat![Plus(3), Mult(2)];
    /// let pats = pat.extend_each(vec![Div(2), Div(3)].into_iter());
    /// assert_eq!(pats, vec![pat![Plus(3), Mult(2), Div(2)], pat![Plus(3), Mult(2), Div(3)]]);
    /// # }
    /// ```
    pub fn extend_each<T>(&self, iter: T) -> Vec<Self> where T: Iterator<Item=PatternElem> {
        iter.map(|elem| {
            let mut v = self.0.clone();
            v.push(elem);
            Pattern::new(v)
        }).collect()
    }
}

impl Display for Pattern {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        for (i, elem) in self.0.iter().enumerate() {
            if i != 0 {
                try!(write!(fmt, ", "));
            }

            try!(write!(fmt, "{}", elem));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PatternElem::*;
    use super::Pattern;

    #[test]
    fn fmt_pat_elem_plus() {
        assert_eq!("+0", format!("{}", Plus(0)));
        assert_eq!("+4", format!("{}", Plus(4)));
    }

    #[test]
    fn fmt_pat_elem_minus() {
        assert_eq!("-1", format!("{}", Plus(-1)));
        assert_eq!("-4", format!("{}", Plus(-4)));
    }

    #[test]
    fn fmt_pat_elem_mult() {
        assert_eq!("*-4", format!("{}", Mult(-4)));
        assert_eq!("*4", format!("{}", Mult(4)));
    }

    #[test]
    fn fmt_pat_elem_div() {
        assert_eq!("/-4", format!("{}", Div(-4)));
        assert_eq!("/4", format!("{}", Div(4)));
    }

    #[test]
    fn fmt_pat_elem_mod() {
        assert_eq!("%-4", format!("{}", Mod(-4)));
        assert_eq!("%4", format!("{}", Mod(4)));
    }

    #[test]
    fn fmt_pat_elem_const() {
        assert_eq!("=-4", format!("{}", Const(-4)));
        assert_eq!("=4", format!("{}", Const(4)));
    }

    #[test]
    fn fmt_pat_elem_square() {
        assert_eq!("^2", format!("{}", Square));
    }

    #[test]
    fn fmt_pat_elem_cube() {
        assert_eq!("^3", format!("{}", Cube));
    }

    #[test]
    fn fmt_pat_elem_square_root() {
        assert_eq!("root 2", format!("{}", SquareRoot));
    }

    #[test]
    fn fmt_pat_elem_cube_root() {
        assert_eq!("root 3", format!("{}", CubeRoot));
    }

    #[test]
    fn fmt_pat() {
        assert_eq!("", format!("{}", Pattern::empty()));
        assert_eq!("+4", format!("{}", pat![Plus(4)]));
        assert_eq!("+4, %-6", format!("{}", pat![Plus(4), Mod(-6)]));
        assert_eq!("+4, %-6, -12, *42, /3, =9", format!("{}", pat![Plus(4), Mod(-6), Plus(-12), Mult(42), Div(3), Const(9)]));
        assert_eq!("^2, root 2, ^3, root 3", format!("{}", pat![Square, SquareRoot, Cube, CubeRoot]));
    }
}
