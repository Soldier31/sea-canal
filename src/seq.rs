use std::fmt::{Display, Formatter, Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SeqElem {
    Plus(i32),
    Mult(i32),
    Div(i32),
    Mod(i32),
    Const(i32),
}


impl Display for SeqElem {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            SeqElem::Plus(i) if i < 0 => write!(fmt, "- {}", i.abs()),
            SeqElem::Plus(i) => write!(fmt, "+ {}", i),
            SeqElem::Mult(i) => write!(fmt, "* {}", i),
            SeqElem::Div(i) => write!(fmt, "/ {}", i),
            SeqElem::Mod(i) => write!(fmt, "% {}", i),
            SeqElem::Const(i) => write!(fmt, "= {}", i),
        }
    }
}

pub struct Seq(Vec<SeqElem>);

impl Seq {
    pub fn new(elems: Vec<SeqElem>) -> Self {
        Seq(elems)
    }
}

impl Display for Seq {
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
    use super::SeqElem::*;
    use super::Seq;

    #[test]
    fn fmt_seq_elem_plus() {
        assert_eq!("+ 0", format!("{}", Plus(0)));
        assert_eq!("+ 4", format!("{}", Plus(4)));
    }

    #[test]
    fn fmt_seq_elem_minus() {
        assert_eq!("- 1", format!("{}", Plus(-1)));
        assert_eq!("- 4", format!("{}", Plus(-4)));
    }

    #[test]
    fn fmt_seq_elem_mult() {
        assert_eq!("* -4", format!("{}", Mult(-4)));
        assert_eq!("* 4", format!("{}", Mult(4)));
    }

    #[test]
    fn fmt_seq_elem_div() {
        assert_eq!("/ -4", format!("{}", Div(-4)));
        assert_eq!("/ 4", format!("{}", Div(4)));
    }

    #[test]
    fn fmt_seq_elem_mod() {
        assert_eq!("% -4", format!("{}", Mod(-4)));
        assert_eq!("% 4", format!("{}", Mod(4)));
    }

    #[test]
    fn fmt_seq_elem_const() {
        assert_eq!("= -4", format!("{}", Const(0)));
        assert_eq!("= 4", format!("{}", Const(4)));
    }

    #[test]
    fn fmt_seq() {
        assert_eq!("", format!("{}", Seq(Vec::new())));
        assert_eq!("+ 4", format!("{}", Seq(vec![Plus(4)])));
        assert_eq!("+ 4, % -6", format!("{}", Seq::new(vec![Plus(4), Mod(-6)])));
        assert_eq!("+ 4, % -6, - 12, * 42, / 3, = 9", format!("{}", Seq(vec![Plus(4), Mod(-6), Plus(-12), Mult(42), Div(3), Const(9)])));
    }
}
