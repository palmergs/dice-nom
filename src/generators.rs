
use super::results::{Pool, Value};

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
    CMP(SuccGenerator),
}

#[derive(Debug, PartialEq)]
pub struct SuccGenerator {
    pub hits: HitsGenerator,
    pub op: Option<SuccessOp>,
}

#[derive(Debug, PartialEq)]
pub enum SuccessOp {
    TargetSucc(i32),
    TargetSuccNext(i32, i32),
}

#[derive(Debug, PartialEq)]
pub struct HitsGenerator {
    pub expr: ExprGenerator,
    pub op: Option<TargetOp>,
}

#[derive(Debug, PartialEq)]
pub enum TargetOp {
    TargetHigh(i32),
    TargetLow(i32),
}

#[derive(Debug, PartialEq)]
pub struct ExprGenerator {
    pub terms: Vec<ArithTermGenerator>
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArithOp {
    ImplicitAdd,
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArithTermGenerator {
    pub op: ArithOp,
    pub term: TermGenerator,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TermGenerator {
    Pool(PoolGenerator),
    Constant(i32)
}

#[derive(Debug, PartialEq, Clone)]
pub struct PoolGenerator {
    pub count: i32,
    pub range: i32,
    pub op: Option<PoolOp>
}

impl PoolGenerator {
    pub fn generate(&self) -> Pool {
        let mut pool = Pool::new();
        for _ in 0..self.count {
            let val = Value::random(self.range, false);
            pool.values.push(val);
            pool.kept += 1;
            pool.sum += val.sum;
            if let Some(op) = &self.op {
                op.apply_last(&mut pool);
            }
        }

        if let Some(op) = &self.op {
            op.apply_all(&mut pool);
        }

        pool
    }
}

#[derive(Debug, PartialEq, Clone)]
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

impl PoolOp {

    /// apply_last modifies the pool based on the current operator.
    /// Some operators do not act on individual values and are skipped.
    /// 
    /// * Examples
    /// 
    /// ```
    /// use dice_nom::generators::PoolOp;
    /// use dice_nom::results::{ Value, Pool };
    /// let val = Value{ value: 6, range: 6, add: 0, constant: false, bonus: false, keep: true, sum: 6 };
    /// let mut pool = Pool{ values: vec![val], kept: 1, bonus: 0, sum: 6 };
    /// PoolOp::ExplodeEach(None).apply_last(&mut pool);
    /// assert_eq!(pool.values.len(), 2); // value is max so it should "explode"
    /// assert_eq!(pool.bonus, 1); // rerolled value is considered bonus
    /// assert_eq!(pool.kept, 2); // all values are kept
    /// assert!(pool.sum > 6); // new roll is added to existing roll
    /// 
    /// let mut pool = Pool{ values: vec![val], kept: 1, bonus: 0, sum: 6 };
    /// PoolOp::ExplodeEachUntil(None).apply_last(&mut pool);
    /// assert!(pool.values.len() >= 2); // value is max so it should "explode"; may continue to explode
    /// 
    /// let mut pool = Pool{ values: vec![val], kept: 1, bonus: 0, sum: 6 };
    /// PoolOp::AddEach(Some(4)).apply_last(&mut pool);
    /// assert_eq!(pool.sum, 10);
    /// assert_eq!(pool.values[0].add, 4);
    /// assert_eq!(pool.values[0].sum, 10);
    /// 
    /// let mut pool = Pool{ values: vec![val], kept: 1, bonus: 0, sum: 6 };
    /// PoolOp::SubEach(Some(4)).apply_last(&mut pool);
    /// assert_eq!(pool.sum, 2);
    /// assert_eq!(pool.values[0].add, -4);
    /// assert_eq!(pool.values[0].sum, 2);
    /// ```
    pub fn apply_last(&self, pool: &mut Pool) {
        if pool.values.len() == 0 {
            return
        }

        match self {
            PoolOp::ExplodeEach(n) => {
                let last = *pool.values.last().unwrap();
                let n = n.unwrap_or(last.range);
                if last.value >= n {
                    let new_roll = Value::random(last.range, true);
                    pool.values.push(new_roll);
                    pool.bonus += 1;
                    pool.kept += 1;
                    pool.sum += new_roll.sum;
                }
            },
            PoolOp::ExplodeEachUntil(n) => {
                loop {
                    let last = *pool.values.last().unwrap();
                    let n = n.unwrap_or(last.range);
                    if last.value >= n {
                        let new_roll = Value::random(last.range, true);
                        pool.values.push(new_roll);
                        pool.bonus += 1;
                        pool.kept += 1;
                        pool.sum += new_roll.sum;
                    } else {
                        break
                    }
                }
            },
            PoolOp::AddEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = n.unwrap_or(1);
                last.add = n;
                last.sum += n;
                
                pool.values.push(last);
                pool.sum += n;
            }
            PoolOp::SubEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = -1 * n.unwrap_or(1);
                last.add = n;
                last.sum += n;

                pool.values.push(last);
                pool.sum += n;
            }
            _ => ()
        }
    }

    pub fn apply_all(&self, pool: &mut Pool) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {

    }
}
