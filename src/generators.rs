
#[derive(Debug, PartialEq)]
pub struct Generator {
    pub succ: SuccGenerator,
    pub op: Option<ComparisonOp>,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOp {
    GT(SuccGenerator),
    GE(SuccGenerator),
    LT(SuccGenerator),
    LE(SuccGenerator),
    EQ(SuccGenerator),
}

#[derive(Debug, PartialEq)]
pub struct SuccGenerator {
    pub expr: ExprGenerator,
    pub op: Option<SuccessOp>,
}

#[derive(Debug, PartialEq)]
pub enum SuccessOp {
    TargetSucc(i32),
    TargetSuccNext(i32, i32),
}

#[derive(Debug, PartialEq)]
pub struct ExprGenerator {

}

#[derive(Debug, PartialEq)]
pub struct HitsGenerator {
    pub term: TermGenerator,
    pub op: Option<TargetOp>,
}

#[derive(Debug, PartialEq)]
pub enum TargetOp {
    TargetHigh(i32),
    TargetLow(i32),
}

#[derive(Debug, PartialEq)]
pub enum TermGenerator {
    Pool(PoolGenerator),
    Constant(i32)
}

#[derive(Debug, PartialEq)]
pub struct PoolGenerator {
    pub count: i32,
    pub range: i32,
    pub op: Option<PoolOp>
}

#[derive(Debug, PartialEq)]
pub enum PoolOp {
    Explode(Option<i32>),
    ExplodeUntil(Option<i32>),
    ExplodeEach(Option<i32>),
    ExplodeEachUntil(Option<i32>),
    AddEach(Option<i32>),
    SubEach(Option<i32>),
    TakeMid(i32),
    TakeLow(i32),
    TakeHigh(i32),
    Disadvantage,
    Advantage,
    BestGroup,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let gen = Generator{
            succ: SuccGenerator{
                expr: ExprGenerator{},
                op: None,
            },
            op: None,
        };
    }
}
