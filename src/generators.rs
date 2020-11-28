
use super::results::{Results, Pool, Value};

#[derive(Debug, PartialEq)]
pub struct Generator {
    pub succ: SuccGenerator,
    pub op: Option<ComparisonOp>,
}

impl Generator {
    /// generate
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
                    let val = if lhs.sum() > rhs.sum() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }

                ComparisonOp::GE(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.sum() >= rhs.sum() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }
                
                ComparisonOp::LT(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.sum() < rhs.sum() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }
                
                ComparisonOp::LE(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.sum() <= rhs.sum() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }    

                ComparisonOp::EQ(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.sum() == rhs.sum() {
                        1
                    } else {
                        0
                    };
                    (Some(rhs), val)
                }                
                
                ComparisonOp::CMP(rhs) => {
                    let rhs = rhs.generate();
                    let val = if lhs.sum() < rhs.sum() {
                        -1
                    } else if lhs.sum() > rhs.sum() {
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

#[derive(Debug, PartialEq)]
pub struct SuccGenerator {
    pub hits: HitsGenerator,
    pub op: Option<SuccessOp>,
}

impl SuccGenerator {
    pub fn generate(&self) -> Pool {
        let mut pool = self.hits.generate();
        match &self.op {
            Some(op) => match op {
                SuccessOp::TargetSucc(n) => {
                    if pool.sum() >= *n {
                        pool.value = pool.sum() - n + 1;
                    }
                    pool
                }
                SuccessOp::TargetSuccNext(n, m) => {
                    if pool.sum() >= *n {
                        pool.value = ((pool.sum() - n) % m) + 1;
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

#[derive(Debug, PartialEq)]
pub struct HitsGenerator {
    pub expr: ExprGenerator,
    pub op: Option<TargetOp>,
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
                        pool.values[idx].hit = pool.values[idx].sum() >= *n;
                    }
                    pool
                }
                TargetOp::TargetLow(n) => {
                    for idx in 0..pool.count() {
                        pool.values[idx].hit = pool.values[idx].sum() <= *n;
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

#[derive(Debug, PartialEq)]
pub struct ExprGenerator {
    pub terms: Vec<ArithTermGenerator>
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

#[derive(Debug, PartialEq, Clone)]
pub struct ArithTermGenerator {
    pub op: ArithOp,
    pub term: TermGenerator,
}

impl ArithTermGenerator {
    pub fn generate(&self) -> Pool {
        let mut pool = self.term.generate();
        match &self.op {
            ArithOp::Sub => {
                for idx in 0..pool.count() {
                    pool.values[idx].mul = -1;
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

impl TermGenerator {
    pub fn generate(&self) -> Pool {
        match self {
            TermGenerator::Pool(pg) => pg.generate(),
            TermGenerator::Constant(n) => Pool{ values: vec![ Value::constant(*n)], value: 0 }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PoolGenerator {
    pub count: i32,
    pub range: i32,
    pub op: Option<PoolOp>
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

impl PoolOp {

    /// apply_last modifies the pool based on the current operator.
    /// Some operators do not act on individual values and are skipped.
    /// 
    /// * Examples
    /// 
    /// ```
    /// use dice_nom::generators::PoolOp;
    /// use dice_nom::results::{ Value, Pool };
    /// let val = Value{ value: 6, range: 6, add: 0, mul: 1, constant: false, bonus: false, keep: true, hit: false };
    /// 
    /// let mut pool = Pool{ values: vec![val], value: 0 };
    /// PoolOp::ExplodeEach(None).apply_last(&mut pool);
    /// assert_eq!(pool.count(), 2); // value is max so it should "explode"
    /// assert_eq!(pool.bonus(), 1); // rerolled value is considered bonus
    /// assert_eq!(pool.kept(), 2); // all values are kept
    /// assert!(pool.sum() > 6); // new roll is added to existing roll
    /// 
    /// let mut pool = Pool{ values: vec![val], value: 0 };
    /// PoolOp::ExplodeEachUntil(None).apply_last(&mut pool);
    /// assert!(pool.count() >= 2); // value is max so it should "explode"; may continue to explode
    /// 
    /// let mut pool = Pool{ values: vec![val], value: 0 };
    /// PoolOp::AddEach(Some(4)).apply_last(&mut pool);
    /// assert_eq!(pool.sum(), 10);
    /// assert_eq!(pool.values[0].add, 4);
    /// assert_eq!(pool.values[0].sum(), 10);
    /// 
    /// let mut pool = Pool{ values: vec![val], value: 0 };
    /// PoolOp::SubEach(Some(4)).apply_last(&mut pool);
    /// assert_eq!(pool.sum(), 2);
    /// assert_eq!(pool.values[0].add, -4);
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
            },
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
            },
            PoolOp::AddEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = n.unwrap_or(1);
                last.add = n;

                pool.values.push(last);
            }
            PoolOp::SubEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = -1 * n.unwrap_or(1);
                last.add = n;

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
    /// let val1 = Value{ value: 6, range: 6, add: 0, mul: 1, constant: false, bonus: false, keep: true, hit: false };
    /// let val2 = Value{ value: 5, range: 6, add: 0, mul: 1, constant: false, bonus: false, keep: true, hit: false };
    /// let val3 = Value{ value: 1, range: 6, add: 0, mul: 1, constant: false, bonus: false, keep: true, hit: false };
    /// let val4 = Value{ value: 6, range: 6, add: 0, mul: 1, constant: false, bonus: false, keep: true, hit: false };
    /// let val5 = Value{ value: 1, range: 6, add: 0, mul: 1, constant: false, bonus: false, keep: true, hit: false };
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2], value: 0 };
    /// PoolOp::Explode(Some(5)).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 2);
    /// assert_eq!(pool.kept(), 4);
    /// assert!(pool.sum() >= 13);
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2], value: 0 };
    /// PoolOp::ExplodeUntil(Some(5)).apply_all(&mut pool);
    /// assert!(pool.count() >= 4);
    /// assert!(pool.bonus() >= 2);
    /// assert!(pool.kept() >= 4);
    /// assert!(pool.sum() >= 13);
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2, val3, val4], value: 0 };
    /// PoolOp::TakeHigh(2).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 12);
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2, val3, val4], value: 0 };
    /// PoolOp::TakeLow(2).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 6);
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2, val3, val4], value: 0 };
    /// PoolOp::TakeMid(2).apply_all(&mut pool);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 11);
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2, val3], value: 0 };
    /// let old_sum = pool.sum();
    /// PoolOp::Advantage.apply_all(&mut pool);
    /// assert_eq!(pool.count(), 6);
    /// assert_eq!(pool.bonus(), 3);
    /// assert_eq!(pool.kept(), 3);
    /// assert!(old_sum <= pool.sum());
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2, val3], value: 0 };
    /// let old_sum = pool.sum();
    /// PoolOp::Disadvantage.apply_all(&mut pool);
    /// assert_eq!(pool.count(), 6);
    /// assert_eq!(pool.bonus(), 3);
    /// assert_eq!(pool.kept(), 3);
    /// assert!(old_sum >= pool.sum());
    /// 
    /// let mut pool = Pool{ values: vec![val1, val2, val3, val4, val5], value: 0 };
    /// PoolOp::BestGroup.apply_all(&mut pool);
    /// assert_eq!(pool.count(), 5);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 12);
    /// 
    /// let mut pool = Pool{ values: vec![val2, val3, val4, val5], value: 0 };
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
                    pool.values[idx].keep = idx < take;
                    
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
                    pool.values[idx].keep = idx >= skip_start && idx < skip_end;
                }                
            }

            PoolOp::TakeHigh(take) => {
                let take = *take as usize;
                if cnt <= take {
                    return
                }

                pool.values.sort_by(|a, b| b.value.cmp(&a.value));
                for idx in 0..cnt {
                    pool.values[idx].keep = idx < take;
                }                
            }

            PoolOp::Advantage => {
                let old = pool.sum();
                for _ in 0..cnt {
                    let roll = Value::random(pool.range(), true);
                    pool.values.push(roll);
                }

                if pool.sum() > old * 2 {
                    for idx in 0..cnt {
                        pool.values[idx].keep = false;
                    }
                } else {
                    for idx in cnt..cnt * 2 {
                        pool.values[idx].keep = false;
                    }
                }
            }

            PoolOp::Disadvantage => {
                let old = pool.sum();
                for _ in 0..cnt {
                    let roll = Value::random(pool.range(), true);
                    pool.values.push(roll);
                }

                if pool.sum() > old * 2 {
                    for idx in cnt..cnt * 2 {
                        pool.values[idx].keep = false;
                    }
                } else {
                    for idx in 0..cnt {
                        pool.values[idx].keep = false;
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
                    if val.keep {
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
                        v.keep = false;
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
