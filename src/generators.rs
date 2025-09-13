use super::results::{Pool, Results, Value};
use rand::prelude::*;
use std::cmp::Ordering;
use std::fmt;

/// The top-level generator that can represent complex dice expressions including comparisons.
///
/// A `Generator` consists of a success generator and an optional comparison operator to
/// compare against another generator. This allows for expressions like "3d6 > 2d8+1".
///
/// # Examples
///
/// ```rust
/// use dice_nom::parse;
/// use rand::prelude::*;
///
/// let mut rng = rand::thread_rng();
///
/// // Simple expression without comparison
/// let simple = parse("3d6+4").unwrap();
/// let result = simple.generate(&mut rng);
/// println!("Result: {}", result.sum());
///
/// // Comparison expression
/// let contest = parse("3d6 > 2d8").unwrap();
/// let result = contest.generate(&mut rng);
/// println!("Contest: {}", result.sum()); // 1 if left wins, 0 otherwise
/// ```
#[derive(Debug, PartialEq)]
pub struct Generator {
    /// The primary success generator for the left side of any comparison
    pub succ: SuccGenerator,
    /// Optional comparison operator for comparing against another generator
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
    /// use rand::prelude::*;
    /// let g = Generator{
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
    /// let mut rng = rand::rng();
    /// let pool = g.generate(&mut rng);
    /// ```
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Results {
        let lhs = self.succ.generate(rng);
        let (rhs, value) = match &self.op {
            Some(op) => match op {
                ComparisonOp::GT(rhs) => {
                    let rhs = rhs.generate(rng);
                    let val = if lhs.value() > rhs.value() { 1 } else { 0 };
                    (Some(rhs), val)
                }

                ComparisonOp::GE(rhs) => {
                    let rhs = rhs.generate(rng);
                    let val = if lhs.value() >= rhs.value() { 1 } else { 0 };
                    (Some(rhs), val)
                }

                ComparisonOp::LT(rhs) => {
                    let rhs = rhs.generate(rng);
                    let val = if lhs.value() < rhs.value() { 1 } else { 0 };
                    (Some(rhs), val)
                }

                ComparisonOp::LE(rhs) => {
                    let rhs = rhs.generate(rng);
                    let val = if lhs.value() <= rhs.value() { 1 } else { 0 };
                    (Some(rhs), val)
                }

                ComparisonOp::EQ(rhs) => {
                    let rhs = rhs.generate(rng);
                    let val = if lhs.value() == rhs.value() { 1 } else { 0 };
                    (Some(rhs), val)
                }

                ComparisonOp::CMP(rhs) => {
                    let rhs = rhs.generate(rng);
                    let val = match lhs.value().cmp(&rhs.value()) {
                        Ordering::Less => -1,
                        Ordering::Greater => 1,
                        Ordering::Equal => 0,
                    };
                    (Some(rhs), val)
                }
            },
            None => (None, 0),
        };
        Results { lhs, rhs, value }
    }
}

/// Comparison operators for comparing two dice expressions.
///
/// Each variant contains the right-hand side generator to compare against.
/// The comparison returns 1 for true, 0 for false, except CMP which returns
/// -1, 0, or 1 for less than, equal, or greater than respectively.
///
/// # Examples
///
/// ```rust
/// use dice_nom::parse;
/// use rand::prelude::*;
///
/// let mut rng = rand::thread_rng();
///
/// // Greater than comparison
/// let gt_test = parse("3d6 > 10").unwrap();
/// let result = gt_test.generate(&mut rng);
/// // result.sum() will be 1 if 3d6 > 10, otherwise 0
/// ```
#[derive(Debug, PartialEq)]
pub enum ComparisonOp {
    /// Greater than (>)
    GT(SuccGenerator),
    /// Greater than or equal (>=)
    GE(SuccGenerator),
    /// Less than (<)
    LT(SuccGenerator),
    /// Less than or equal (<=)
    LE(SuccGenerator),
    /// Equal (=)
    EQ(SuccGenerator),
    /// Three-way comparison (<=>)
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

/// Generator for success-based dice systems.
///
/// A success generator evaluates hits from a dice pool and applies success thresholds.
/// This is commonly used in systems where you need to achieve a certain total to succeed,
/// with additional successes for exceeding thresholds.
///
/// # Examples
///
/// ```rust
/// use dice_nom::parse;
/// use rand::prelude::*;
///
/// let mut rng = rand::thread_rng();
///
/// // Success if total >= 12
/// let success_test = parse("3d6{12}").unwrap();
/// let result = success_test.generate(&mut rng);
///
/// // Success levels: 1 success at 15, +1 for every 5 above
/// let level_test = parse("2d10{15,5}").unwrap();
/// let result = level_test.generate(&mut rng);
/// ```
#[derive(Debug, PartialEq)]
pub struct SuccGenerator {
    /// The hits generator that produces the base dice rolls
    pub hits: HitsGenerator,
    /// Optional success operation for threshold-based success counting
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
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Pool {
        let mut pool = self.hits.generate(rng);
        match &self.op {
            Some(op) => match op {
                SuccessOp::TargetSucc(n) => {
                    if pool.sum() >= *n {
                        pool.set_total(pool.sum() - n + 1);
                    } else {
                        pool.set_total(0);
                    }
                    pool
                }
                SuccessOp::TargetSuccNext(n, m) => {
                    if pool.sum() >= *n {
                        pool.set_total(((pool.sum() - n) / m) + 1);
                    } else {
                        pool.set_total(0);
                    }
                    pool
                }
            },
            None => pool,
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
            SuccessOp::TargetSuccNext(n, m) => write!(f, "{{{}, {}}}", n, m),
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
    /// use rand::prelude::*;
    /// let g = HitsGenerator{
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
    /// let mut rng = rand::rng();
    /// let pool = g.generate(&mut rng);
    /// assert!(pool.hits() > 0);
    /// ```
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Pool {
        let mut pool = self.expr.generate(rng);
        match &self.op {
            Some(op) => match op {
                TargetOp::TargetHigh(n) => {
                    for idx in 0..pool.count() {
                        let b = pool.values[idx].sum().abs() >= *n;
                        pool.values[idx].set_hit(b);
                    }
                    pool.set_total(pool.sum());
                    pool
                }
                TargetOp::TargetLow(n) => {
                    for idx in 0..pool.count() {
                        let b = pool.values[idx].sum().abs() <= *n;
                        pool.values[idx].set_hit(b);
                    }
                    pool.set_total(pool.sum());
                    pool
                }
            },
            None => pool,
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
    pub terms: Vec<ArithTermGenerator>,
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
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Pool {
        let mut pool = Pool::new();
        for t in self.terms.iter() {
            pool.values.append(&mut t.generate(rng).values);
        }
        pool.set_total(pool.sum());
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
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Pool {
        let mut pool = self.term.generate(rng);
        match &self.op {
            ArithOp::Sub => {
                for idx in 0..pool.count() {
                    pool.values[idx].mark_penalty();
                }
                pool
            }
            _ => pool,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TermGenerator {
    Pool(PoolGenerator),
    Constant(i32),
}

impl fmt::Display for TermGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TermGenerator::Pool(pg) => write!(f, "{}", pg),
            TermGenerator::Constant(n) => write!(f, "{}", n),
        }
    }
}

impl TermGenerator {
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Pool {
        match self {
            TermGenerator::Pool(pg) => pg.generate(rng),
            TermGenerator::Constant(n) => Pool::new_with_values(vec![Value::constant(*n)]),
        }
    }
}

/// Generator for a pool of dice with optional operations.
///
/// This is the fundamental building block representing "XdY" notation, where X is the
/// count of dice and Y is the range (number of sides). Optional operations can modify
/// how the dice behave (exploding, advantage, etc.).
///
/// # Examples
///
/// ```rust
/// use dice_nom::generators::PoolGenerator;
/// use rand::prelude::*;
///
/// let mut rng = rand::thread_rng();
///
/// // Simple 3d6
/// let basic = PoolGenerator { count: 3, range: 6, op: None };
/// let result = basic.generate(&mut rng);
///
/// // Using the convenience function
/// use dice_nom::roller;
/// let exploding = roller(2, 8, Some("!"));
/// let result = exploding.generate(&mut rng);
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct PoolGenerator {
    /// Number of dice to roll
    pub count: i32,
    /// Number of sides on each die (range 1 to this value)
    pub range: i32,
    /// Optional operation to apply to the dice pool
    pub op: Option<PoolOp>,
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
    /// use rand::prelude::*;
    /// let mut rng = rand::rng();
    /// let g = PoolGenerator{ count: 3, range: 6, op: Some(PoolOp::ExplodeEach(None)) };
    /// let pool = g.generate(&mut rng);
    /// assert!(pool.count() >= 3);
    /// ```
    pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Pool {
        let mut pool = Pool::new();
        for _ in 0..self.count {
            let val = Value::random(self.range, false, rng);
            pool.values.push(val);
            if let Some(op) = &self.op {
                op.apply_last(&mut pool, rng);
            }
        }

        if let Some(op) = &self.op {
            op.apply_all(&mut pool, rng);
        }

        pool
    }
}

/// Operations that can be applied to dice pools.
///
/// These operations modify how dice behave after being rolled, such as exploding
/// on maximum values, keeping only certain dice, or adding modifiers.
///
/// # Examples
///
/// ```rust
/// use dice_nom::roller;
/// use rand::prelude::*;
///
/// let mut rng = rand::thread_rng();
///
/// // Exploding dice - reroll on max, once per pool
/// let exploding = roller(3, 6, Some("!"));
///
/// // Advantage - roll twice, keep higher
/// let advantage = roller(1, 20, Some("ADV"));
///
/// // Keep highest 3 of 4 dice
/// let drop_lowest = roller(4, 6, Some("^3"));
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum PoolOp {
    /// Explode: reroll if all dice are max (or >= threshold)
    Explode(Option<i32>),
    /// Explode until: keep rerolling while all dice are max
    ExplodeUntil(Option<i32>),
    /// Explode each: reroll each die that is max (or >= threshold)
    ExplodeEach(Option<i32>),
    /// Explode each until: keep rerolling each die while it's max
    ExplodeEachUntil(Option<i32>),
    /// Add the given value to each die
    AddEach(Option<i32>),
    /// Subtract the given value from each die
    SubEach(Option<i32>),
    /// Keep the middle N dice (drop highest and lowest)
    TakeMid(i32),
    /// Keep the lowest N dice
    TakeLow(i32),
    /// Keep the highest N dice
    TakeHigh(i32),
    /// Roll twice, keep the lower result
    Disadvantage,
    /// Roll twice, keep the higher result
    Advantage,
    /// Keep the largest group of matching dice values
    BestGroup,
}

impl fmt::Display for PoolOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoolOp::Explode(n) => {
                if let Some(n) = *n {
                    if n > 1 {
                        write!(f, "!{}", n)
                    } else {
                        write!(f, "!")
                    }
                } else {
                    write!(f, "!")
                }
            }

            PoolOp::ExplodeUntil(n) => {
                if let Some(n) = *n {
                    if n > 1 {
                        write!(f, "!!{}", n)
                    } else {
                        write!(f, "!!")
                    }
                } else {
                    write!(f, "!!")
                }
            }

            PoolOp::ExplodeEach(n) => {
                if let Some(n) = *n {
                    if n > 1 {
                        write!(f, "*{}", n)
                    } else {
                        write!(f, "*")
                    }
                } else {
                    write!(f, "*")
                }
            }

            PoolOp::ExplodeEachUntil(n) => {
                if let Some(n) = *n {
                    if n > 1 {
                        write!(f, "**{}", n)
                    } else {
                        write!(f, "**")
                    }
                } else {
                    write!(f, "**")
                }
            }

            PoolOp::AddEach(n) => {
                if let Some(n) = n {
                    write!(f, "++{}", n)
                } else {
                    write!(f, "++")
                }
            }

            PoolOp::SubEach(n) => {
                if let Some(n) = n {
                    write!(f, "--{}", n)
                } else {
                    write!(f, "--")
                }
            }

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
    /// use rand::prelude::*;
    /// let mut rng = rand::rng();
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6)]);
    /// PoolOp::ExplodeEach(None).apply_last(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 2); // value is max so it should "explode"
    /// assert_eq!(pool.bonus(), 1); // rerolled value is considered bonus
    /// assert_eq!(pool.kept(), 2); // all values are kept
    /// assert!(pool.sum() > 6); // new roll is added to existing roll
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6)]);
    /// PoolOp::ExplodeEachUntil(None).apply_last(&mut pool, &mut rng);
    /// assert!(pool.count() >= 2); // value is max so it should "explode"; may continue to explode
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(4)]);
    /// PoolOp::AddEach(Some(4)).apply_last(&mut pool, &mut rng);
    /// assert_eq!(pool.sum(), 8);
    /// assert_eq!(pool.values[0].modifier(), 4);
    /// assert_eq!(pool.values[0].sum(), 8);
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(4)]);
    /// PoolOp::SubEach(Some(4)).apply_last(&mut pool, &mut rng);
    /// assert_eq!(pool.sum(), 0);
    /// assert_eq!(pool.values[0].modifier(), -4);
    /// assert_eq!(pool.values[0].sum(), 0);
    /// ```
    pub fn apply_last<R: Rng + ?Sized>(&self, pool: &mut Pool, rng: &mut R) {
        if pool.count() == 0 {
            return;
        }

        match self {
            PoolOp::ExplodeEach(n) => {
                let n = self.safe_n(n, pool.last_range());
                if pool.last_value() >= n {
                    let new_roll = Value::random(pool.last_range(), true, rng);
                    pool.values.push(new_roll);
                }
            }

            PoolOp::ExplodeEachUntil(n) => loop {
                let n = self.safe_n(n, pool.last_range());
                if pool.last_value() >= n {
                    let new_roll = Value::random(pool.last_range(), true, rng);
                    pool.values.push(new_roll);
                } else {
                    break;
                }
            },

            PoolOp::AddEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = n.unwrap_or(1);
                last.set_modifier(n);
                pool.values.push(last);
            }

            PoolOp::SubEach(n) => {
                let mut last = pool.values.pop().unwrap();
                let n = -n.unwrap_or(1);
                last.set_modifier(n);
                pool.values.push(last);
            }
            _ => (),
        }
    }

    fn safe_n(&self, n: &Option<i32>, range: i32) -> i32 {
        match *n {
            Some(n) => {
                if n <= 1 || n > range {
                    range
                } else {
                    n
                }
            }
            None => range,
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
    /// use rand::prelude::*;
    /// let mut rng = rand::rng();
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5)]);
    /// PoolOp::Explode(Some(5)).apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 2);
    /// assert_eq!(pool.kept(), 4);
    /// assert!(pool.sum() >= 13);
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5)]);
    /// PoolOp::ExplodeUntil(Some(5)).apply_all(&mut pool, &mut rng);
    /// assert!(pool.count() >= 4);
    /// assert!(pool.bonus() >= 2);
    /// assert!(pool.kept() >= 4);
    /// assert!(pool.sum() >= 13);
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5), Value::d6(1), Value::d6(4)]);
    /// PoolOp::TakeHigh(2).apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 11);
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5), Value::d6(1), Value::d6(4)]);
    /// PoolOp::TakeLow(2).apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 5);
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5), Value::d6(1), Value::d6(4)]);
    /// PoolOp::TakeMid(2).apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 4);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 9);
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5), Value::d6(1)]);
    /// let old_sum = pool.sum();
    /// PoolOp::Advantage.apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 6);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 3);
    /// assert!(old_sum <= pool.sum());
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5), Value::d6(1)]);
    /// let old_sum = pool.sum();
    /// PoolOp::Disadvantage.apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 6);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 3);
    /// assert!(old_sum >= pool.sum());
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(6), Value::d6(5), Value::d6(6), Value::d6(2), Value::d6(2)]);
    /// PoolOp::BestGroup.apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.count(), 5);
    /// assert_eq!(pool.bonus(), 0);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 12);
    ///
    /// let mut pool = Pool::new_with_values(vec![Value::d6(5), Value::d6(6), Value::d6(2), Value::d6(2)]);
    /// PoolOp::BestGroup.apply_all(&mut pool, &mut rng);
    /// assert_eq!(pool.kept(), 2);
    /// assert_eq!(pool.sum(), 4);
    /// ```
    pub fn apply_all<R: Rng + ?Sized>(&self, pool: &mut Pool, rng: &mut R) {
        let cnt = pool.count();
        if cnt == 0 {
            return;
        }

        match self {
            PoolOp::Explode(n) => {
                let range = pool.range();
                let n = self.safe_n(n, range);
                let explode = pool.values.iter().all(|v| v.value >= n);
                if explode {
                    for _ in 0..cnt {
                        let roll = Value::random(range, true, rng);
                        pool.values.push(roll);
                    }
                }
            }

            PoolOp::ExplodeUntil(n) => {
                let range = pool.range();
                let n = self.safe_n(n, range);
                let mut explode = pool.values.iter().all(|v| v.value >= n);
                while explode {
                    for _ in 0..cnt {
                        pool.values.push(Value::random(range, true, rng));
                        if pool.last_value() < n {
                            explode = false;
                        }
                    }
                }
            }

            PoolOp::TakeLow(take) => {
                let take = *take as usize;
                if cnt <= take {
                    return;
                }

                pool.values.sort_by(|a, b| a.value.cmp(&b.value));
                for idx in 0..cnt {
                    if idx >= take {
                        pool.values[idx].mark_discarded();
                    }
                }
            }

            PoolOp::TakeMid(take) => {
                let take = *take as usize;
                if cnt <= take {
                    return;
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
                    return;
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
                    let roll = Value::random(range, false, rng);
                    pool.values.push(roll);
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
                    let roll = Value::random(range, false, rng);
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
                let values = pool.values();
                for val in values.into_iter() {
                    if let Some(n) = val {
                        if last_val == n {
                            curr_run += 1;
                            if curr_run > max_run {
                                max_run = curr_run;
                                max_val = last_val;
                            }
                        } else {
                            last_val = n;
                            curr_run = 0;
                        }
                    }
                }

                for v in &mut pool.values {
                    if v.value != max_val {
                        v.mark_discarded();
                    }
                }
            }
            _ => (),
        }
    }
}
