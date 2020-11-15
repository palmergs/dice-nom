use std::fmt;

pub(self) mod roller {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Op {
        Explode,
        ExplodeEach,
        ExplodeUntil,
        ExplodeUntilEach,
        AddEach,
        SubEach,
        Critical,
        Disadvantage,
        Advantage,
        TakeMid,
        TakeLow,
        TakeHigh,
    }

    impl fmt::Display for Op {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Op::Explode => write!(f, "!"),
                Op::ExplodeEach => write!(f, "!!"),
                Op::ExplodeUntil => write!(f, "*"),
                Op::ExplodeUntilEach => write!(f, "**"),
                Op::AddEach => write!(f, "++"),
                Op::SubEach => write!(f, "--"),
                Op::Critical => write!(f, "$"),
                Op::Disadvantage => write!(f, "D"),
                Op::Advantage => write!(f, "A"),
                Op::TakeMid => write!(f, "~"),
                Op::TakeLow => write!(f, "`"),
                Op::TakeHigh => write!(f, "^"),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Expr {
        EVal(i32),
        ERoll(i32, i32, Option<Op>, Option<i32>),
        EAdd(Box<Expr>, Box<Expr>),
        ESub(Box<Expr>, Box<Expr>),
        EPar(Box<Expr>),
        EHalfDown(Box<Expr>),
        EHalfUp(Box<Expr>),
        EList(Vec<Expr>),
        ETargetAbove(Box<Expr>, i32),
        ETargetBelow(Box<Expr>, i32),
    }

    #[cfg(test)]
    mod tests {
        use super::*;

    }
}
