use std::fmt;
use std::num;
use super::results::{Results, Pool, Value};

#[derive(Debug, PartialEq)]
pub struct Generator {
    pub succ: SuccGenerator,
    pub op: Option<ComparisonOp>,
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.succ)?;
        if let Some(op) = &self.op {
            write!(f, " {}", op)?;
        }
        write!(f, "")
    }
}

impl Generator {
    /// generate builds a top-level generator that can compare two 
    /// 
    /// * Example
    /// 
    /// ```
    /// use dice_nom::generators::*;
    /// use dice_nom::results::*;
    /// let gen = Generator{
    ///     succ: SuccGenerator{
    ///         hits: HitsGenerator{ 
    ///             expr: ExprGenerator{
    ///                 terms: vec![ArithTermGenerator{
    ///                     op: ArithOp::ImplicitAdd,
    ///                     term: TermGenerator::Pool(PoolGenerator{
    ///                         count: 12,
    ///                         range: 6,
    ///                         op: None
    ///                     })
    ///                 }]
    ///             },
    ///             op: None
    ///         },
    ///         op: None
    ///     },
    ///     op: None 
    /// };
    /// let pool = gen.generate();
    /// ```
    pub fn generate(&self) -> Results {
        let lhs = self.succ.generate();
        let (rhs, value) = match &self.op {
            Some(op) => match op {
                ComparisonOp::GT(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.value() > rhs.value() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }

                ComparisonOp::GE(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.value() >= rhs.value() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }
                
                ComparisonOp::LT(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.value() < rhs.value() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }
                
                ComparisonOp::LE(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.value() <= rhs.value() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }    

                ComparisonOp::EQ(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.value() == rhs.value() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }                
                
                ComparisonOp::CMP(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.value() < rhs.value() {
                        -1
                    } else if lhs.value() > rhs.value() {
                        1
                    }else {
                        0
                    };
                    (Some(rhs), val)
                }                 
            },
            None => (None, 0)
        };
        Results{ lhs, rhs, value }
    }
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

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComparisonOp::GT(succ) => write!(f, "> {}", succ),
            ComparisonOp::GE(succ) => write!(f, ">= {}", succ),
            ComparisonOp::LT(succ) => write!(f, "< {}", succ),
            ComparisonOp::LE(succ) => write!(f, "<= {}", succ),
            ComparisonOp::EQ(succ) => write!(f, "= {}", succ),
            ComparisonOp::CMP(succ) => write!(f, "<=> {}", succ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SuccGenerator {
    pub hits: HitsGenerator,
    pub op: Option<SuccessOp>,
}

impl fmt::Display for SuccGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hits)?;
        if let Some(op) = &self.op {
            write!(f, "{}", op)?;
        }
        write!(f, "")
    }
}

impl SuccGenerator {
    /// generate builds a generator that calculates success based on whether
    /// the pool sum is greater than the target number.
    pub fn generate(&self) -> Pool {
        let mut pool = self.hits.generate();
        match &self.op {
            Some(op) => match op {
                SuccessOp::TargetSucc(n) => {
                    if pool.sum() >= *n {
                        pool.set_value(pool.sum() - n + 1);
                    } else {
                        pool.set_value(0);
                    }
                    pool
                }
                SuccessOp::TargetSuccNext(n, m) => {
                    if pool.sum() >= *n {
                        pool.set_value(((pool.sum() - n) / m) + 1);
                    } else {
                        pool.set_value(0);
                    }
                    pool
                }
            },
            None => pool
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SuccessOp {
    TargetSucc(i32),
    TargetSuccNext(i32, i32),
}

impl fmt::Display for SuccessOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuccessOp::TargetSucc(n) => write!(f, "{{{}}}", n),
            SuccessOp::TargetSuccNext(n, m) => write!(f, "{{{}, {}}}", n, m)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HitsGenerator {
    pub expr: ExprGenerator,
    pub op: Option<TargetOp>,
}

impl fmt::Display for HitsGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expr)?;
        if let Some(op) = &self.op {
            write!(f, "{}", op)?;
        }
        write!(f, "")
    }
}

impl HitsGenerator {
    /// generate
    /// 
    /// * Example
    /// 
    /// ```
    /// use dice_nom::generators::*;
    /// use dice_nom::results::*;
    /// let gen = HitsGenerator{ 
    ///     expr: ExprGenerator{
    ///         terms: vec![ArithTermGenerator{
    ///             op: ArithOp::ImplicitAdd,
    ///             term: TermGenerator::Pool(PoolGenerator{
    ///                 count: 12,
    ///                 range: 6,
    ///                 op: None,
    ///             })
    ///         }]
    ///     },
    ///     op: Some(TargetOp::TargetHigh(4)) 
    /// };
    /// let pool = gen.generate();
    /// // TODO: this assertion is a bit of a risk since there's a chance of no hits 
    /// assert!(pool.hits() > 0); 
    /// ```
    pub fn generate(&self) -> Pool {
        let mut pool = self.expr.generate();
        match &self.op {
            Some(op) => match op {
                TargetOp::TargetHigh(n) => {
                    for idx in 0..pool.count() {
                        let b = pool.values[idx].sum().abs() >= *n;
                        pool.values[idx].set_hit(b);
                    }
                    pool
                }
                TargetOp::TargetLow(n) => {
                    for idx in 0..pool.count() {
                        let b = pool.values[idx].sum().abs() <= *n;
                        pool.values[idx].set_hit(b);
                    }
                    pool
                }
            }
            None => pool
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TargetOp {
    TargetHigh(i32),
    TargetLow(i32),
}

impl fmt::Display for TargetOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetOp::TargetHigh(n) => write!(f, "[{}]", n),
            TargetOp::TargetLow(n) => write!(f, "({})", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExprGenerator {
    pub terms: Vec<ArithTermGenerator>
}

impl fmt::Display for ExprGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for t in self.terms.iter() {
            write!(f, "{}", t)?;
        }
        write!(f, "")
    }
}

impl ExprGenerator {
    pub fn generate(&self) -> Pool {
        let mut pool = Pool::new();
        for t in self.terms.iter() {
            pool.values.append(&mut t.generate().values);
        }
        pool
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArithOp {
    ImplicitAdd,
    Add,
    Sub,
}

impl fmt::Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithOp::ImplicitAdd => write!(f, ""),
            ArithOp::Add => write!(f, " + "),
            ArithOp::Sub => write!(f, " - "),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArithTermGenerator {
    pub op: ArithOp,
    pub term: TermGenerator,
}

impl fmt::Display for ArithTermGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.op, self.term)
    }
}

impl ArithTermGenerator {
    pub fn generate(&self) -> Pool {
        let mut pool = self.term.generate();
        match &self.op {
            ArithOp::Sub => {
                for idx in 0..pool.count() {
                    pool.values[idx].mark_penalty();
                }
                pool
            }
            _ => pool
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TermGenerator {
    Pool(PoolGenerator),
    Constant(i32)
}

impl fmt::Display for TermGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TermGenerator::Pool(pg) => write!(f, "{}", pg),
            TermGenerator::Constant(n) => write!(f, "{}", n)
        }
    }
}

impl TermGenerator {
    pub fn generate(&self) -> Pool {
        match self {
            TermGenerator::Pool(pg) => pg.generate(),
            TermGenerator::Constant(n) => Pool::new_with_values(vec![ Value::constant(*n) ])
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PoolGenerator {
    pub count: i32,
    pub range: i32,
    pub op: Option<PoolOp>
}

impl fmt::Display for PoolGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.count, self.range)?;
        if let Some(op) = &self.op {
            write!(f, "{}", op)?;
        }
        write!(f, "")
    }
}

impl PoolGenerator {

    /// generate 
    /// 
    /// * Example
    /// 
    /// ```
    /// use dice_nom::generators::{PoolGenerator, PoolOp};
    /// use dice_nom::results::Pool;
    /// let gen = PoolGenerator{ count: 3, range: 6, op: Some(PoolOp::ExplodeEach(None)) };
    /// let pool = gen.generate();
    /// assert!(pool.count() >= 3);
    /// ```
    pub fn generate(&self) -> Pool {
        let mut pool = Pool::new();
        for _ in 0..self.count {
            let val = Value::random(self.range, false);
            pool.values.push(val);
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

impl fmt::Display for PoolOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoolOp::Explode(n) => if let Some(n) = *n {
                write!(f, "!{}", n)
            } else {
                write!(f, "!")
            },

            PoolOp::ExplodeUntil(n) => if let Some(n) = *n {
                write!(f, "!!{}", n)
            } else {
                write!(f, "!!")
            },

            PoolOp::ExplodeEach(n) => if let Some(n) = *n {
                write!(f, "*{}", n)
            } else {
                write!(f, "*")
            },

            PoolOp::ExplodeEachUntil(n) => if let Some(n) = *n {
                write!(f, "**{}", n)
            } else {
                write!(f, "**")
            },

            PoolOp::AddEach(n) => if let Some(n) = *n {
                write!(f, "++{}", n)
            } else {
                write!(f, "++")
            },

            PoolOp::SubEach(n) => if let Some(n) = *n {
                write!(f, "--{}", n)
            } else {
                write!(f, "--")
            },

            PoolOp::TakeMid(n) => write!(f, "~{}", n),
            PoolOp::TakeLow(n) => write!(f, "`{}", n),
            PoolOp::TakeHigh(n) => write!(f, "^{}", n),
            PoolOp::Disadvantage => write!(f, " DIS"),
            PoolOp::Advantage => write!(f, " ADV"),
            PoolOp::BestGroup => write!(f, "Y"),
        }
    }
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
    /// let val = Value::random_with_value(6, 6, false);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val]);
    /// PoolOp::ExplodeEach(None).apply_last(&mut pool);
    /// assert_eq!(pool.count(), 2); // value is max so it should "explode"
    /// assert_eq!(pool.bonus(), 1); // rerolled value is considered bonus
    /// assert_eq!(pool.kept(), 2); // all values are kept
    /// assert!(pool.sum() > 6); // new roll is added to existing roll
    /// 
    /// let mut pool = Pool::new_with_values(vec![val]);
    /// PoolOp::ExplodeEachUntil(None).apply_last(&mut pool);
    /// assert!(pool.count() >= 2); // value is max so it should "explode"; may continue to explode
    /// 
    /// let mut pool = Pool::new_with_values(vec![val]);
    /// PoolOp::AddEach(Some(4)).apply_last(&mut pool);
    /// assert_eq!(pool.sum(), 10);
    /// assert_eq!(pool.values[0].modifier(), 4);
    /// assert_eq!(pool.values[0].sum(), 10);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val]);
    /// PoolOp::SubEach(Some(4)).apply_last(&mut pool);
    /// assert_eq!(pool.sum(), 2);
    /// assert_eq!(pool.values[0].modifier(), -4);
    /// assert_eq!(pool.values[0].sum(), 2);
    /// ```
    pub fn apply_last(&self, pool: &mut Pool) {
        if pool.count() == 0 {
            return
        }

        match self {
            PoolOp::ExplodeEach(n) => {
                let last = *pool.values.last().unwrap();
                let n = n.unwrap_or(last.range);
                if last.value >= n {
                    let new_roll = Value::random(last.range, true);
                    pool.values.push(new_roll);
                }
            }

            PoolOp::ExplodeEachUntil(n) => {
                loop {
                    let last = *pool.values.last().unwrap();
                    let n = n.unwrap_or(last.range);
                    if last.value >= n {
                        let new_roll = Value::random(last.range, true);
                        pool.values.push(new_roll);
                    } else {
                        break
                    }
                }
            }

            PoolOp::AddEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = n.unwrap_or(1);
                last.set_modifier(n);
                pool.values.push(last);
            }

            PoolOp::SubEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = -1 * n.unwrap_or(1);
                last.set_modifier(n);
                pool.values.push(last);
            }
            _ => ()
        }
    }

    /// apply_all modifies the pool based on the current operator
    /// that may modify the entire dice pool. Some operators only apply to 
    /// individual values and are ignored here.
    ///
    /// * Examples
    /// 
    /// ```
    /// use dice_nom::generators::PoolOp;
    /// use dice_nom::results::{ Value, Pool };
    /// let val1 = Value::random_with_value(6, 6, false);
    /// let val2 = Value::random_with_value(5, 6, false);
    /// let val3 = Value::random_with_value(1, 6, false);
    /// let val4 = Value::random_with_value(6, 6, false);
    /// let val5 = Value::random_with_value(1, 6, false);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2]);
    /// PoolOp::Explode(Some(5)).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 2);
    /// assert_eq!(pool.kept(), 4);
    /// assert!(pool.sum() >= 13);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2]);
    /// PoolOp::ExplodeUntil(Some(5)).apply_all(&mut pool);
    /// assert!(pool.count() >= 4);
    /// assert!(pool.bonus() >= 2);
    /// assert!(pool.kept() >= 4);
    /// assert!(pool.sum() >= 13);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2, val3, val4]);
    /// PoolOp::TakeHigh(2).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 12);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2, val3, val4]);
    /// PoolOp::TakeLow(2).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 6);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2, val3, val4]);
    /// PoolOp::TakeMid(2).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 11);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2, val3]);
    /// let old_sum = pool.sum();
    /// PoolOp::Advantage.apply_all(&mut pool);
    /// assert_eq!(pool.count(), 6);
    /// println!("pool: bonus={} kept={}", pool.bonus(), pool.kept());
    /// assert_eq!(pool.bonus(), 3);
    /// assert_eq!(pool.kept(), 3);
    /// assert!(old_sum <= pool.sum());
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2, val3]);
    /// let old_sum = pool.sum();
    /// PoolOp::Disadvantage.apply_all(&mut pool);
    /// assert_eq!(pool.count(), 6);
    /// assert_eq!(pool.bonus(), 3);
    /// assert_eq!(pool.kept(), 3);
    /// assert!(old_sum >= pool.sum());
    /// 
    /// let mut pool = Pool::new_with_values(vec![val1, val2, val3, val4, val5]);
    /// PoolOp::BestGroup.apply_all(&mut pool);
    /// assert_eq!(pool.count(), 5);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 12);
    /// 
    /// let mut pool = Pool::new_with_values(vec![val2, val3, val4, val5]);
    /// PoolOp::BestGroup.apply_all(&mut pool);
    /// assert_eq!(pool.sum(), 2);
    /// ```
    pub fn apply_all(&self, pool: &mut Pool) {
        let cnt = pool.count();
        if cnt == 0 {
            return
        }

        match self {
            PoolOp::Explode(n) => {
                let range = pool.range();
                let n = n.unwrap_or(range);
                let explode = pool.values.iter().all(|&v| v.value >= n );
                if explode {
                    for _ in 0..cnt {
                        let roll = Value::random(range, true);
                        pool.values.push(roll);
                    }
                }
            }

            PoolOp::ExplodeUntil(n) => {
                let range = pool.range();
                let n = n.unwrap_or(range);
                let mut explode = pool.values.iter().all(|&v| v.value >= n );
                while explode {
                    for _ in 0..cnt {
                        let roll = Value::random(range, true);
                        pool.values.push(roll);
                        if roll.value < n {
                            explode = false;
                        }
                    }
                }
            }

            PoolOp::TakeLow(take) => {
                let take = *take as usize;
                if cnt <= take {
                    return
                }

                pool.values.sort_by(|a, b| a.value.cmp(&b.value));
                for idx in 0..cnt {
                    if idx >= take {
                        pool.values[idx].mark_discarded();
                    }
                }
            }

            PoolOp::TakeMid(take)=> {
                let take = *take as usize;
                if cnt <= take {
                    return
                }

                pool.values.sort_by(|a, b| b.value.cmp(&a.value));
                let skip_start = (cnt - take) / 2;
                let skip_end = skip_start + take;
                for idx in 0..cnt {
                    if idx < skip_start || idx >= skip_end {
                        pool.values[idx].mark_discarded();
                    }
                }                
            }

            PoolOp::TakeHigh(take) => {
                let take = *take as usize;
                if cnt <= take {
                    return
                }

                pool.values.sort_by(|a, b| b.value.cmp(&a.value));
                for idx in 0..cnt {
                    if idx >= take {
                        pool.values[idx].mark_discarded();
                    }
                }                
            }

            PoolOp::Advantage => {
                let old = pool.sum();
                let range = pool.range();
                for _ in 0..cnt {
                    let roll = Value::random(range, true);
                    pool.values.push(roll);
                    println!("pool = {:?}", pool);
                }

                if pool.sum() > old * 2 {
                    for idx in 0..cnt {
                        pool.values[idx].mark_discarded();
                    }
                } else {
                    for idx in cnt..cnt * 2 {
                        pool.values[idx].mark_discarded();
                    }
                }
            }

            PoolOp::Disadvantage => {
                let old = pool.sum();
                let range = pool.range();
                for _ in 0..cnt {
                    let roll = Value::random(range, true);
                    pool.values.push(roll);
                }

                if pool.sum() > old * 2 {
                    for idx in cnt..cnt * 2 {
                        pool.values[idx].mark_discarded();
                    }
                } else {
                    for idx in 0..cnt {
                        pool.values[idx].mark_discarded();
                    }
                }
            }

            PoolOp::BestGroup => {
                pool.values.sort_by(|a, b| b.value.cmp(&a.value));
                let mut last_val = 0;
                let mut max_val = 0;
                let mut max_run = 0;
                let mut curr_run = 0;
                for idx in 0..cnt {
                    let val = pool.values[idx];
                    if !val.is_discarded() {
                        if last_val == val.value {
                            curr_run += 1;
                            if curr_run > max_run {
                                max_run = curr_run;
                                max_val = last_val;
                            }
                        } else {
                            last_val = val.value;
                            curr_run = 0;
                        }
                    }
                }

                for v in &mut pool.values {
                    if v.value != max_val {
                        v.mark_discarded();
                    }
                }
            },
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {

    }
}
