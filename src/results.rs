use rand::Rng;
use serde::Serialize;
use std::fmt;

/// Represents a single physical die roll.
///
/// A `Die` tracks both the physical value rolled and any calculated modifications.
/// The `value` field may be `None` if the die was discarded (e.g., when rolling
/// for non-standard ranges).
///
/// # Examples
///
/// ```rust
/// use dice_nom::results::Die;
/// use rand::prelude::*;
///
/// let mut rng = rand::rng();
/// let (dice, total) = Die::roll(6, &mut rng);
/// assert_eq!(dice.len(), 1);
/// assert!(total >= 1 && total <= 6);
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub struct Die {
    /// value on the physical die
    pub rolled: i32,

    /// range of the physical die
    pub die: i32,

    /// calculated value
    pub value: Option<i32>,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(n) => write!(f, "{}/{} ({})", self.rolled, self.die, n),
            None => write!(f, "{}/{}", self.rolled, self.die),
        }
    }
}

impl Die {
    /// roll dice to achieve an equal distribution over the expected range
    /// For ranges that are "standard" simply generate a die roll. For
    /// percentage dice they are rolled in order (thousands, hundreds, tens, ones)
    /// so the result is more than one die roll. For non-standard values,
    /// a die roll is generated for the next largest standard value and
    /// anything outside that range is discarded. The last die in the vector
    /// should be the one that was kept.
    ///
    /// * Examples:
    ///
    /// ```
    /// use dice_nom::results::Die;
    /// use rand::prelude::*;
    /// let mut rng = rand::rng();
    /// let (dice, value) = Die::roll(1000, &mut rng);
    /// assert_eq!(dice.len(), 3);
    /// assert!(value > 0);
    /// assert!(value <= 1000);
    ///
    /// let (dice, value) = Die::roll(25, &mut rng);
    /// assert_eq!(dice.len(), 2);
    /// assert!(value > 0);
    /// assert!(value <= 25);
    /// ```
    pub fn roll<R: Rng + ?Sized>(range: i32, rng: &mut R) -> (Vec<Die>, i32) {
        let mut dice = Vec::new();
        let mut value = 0_i32;
        match range {
            2 => {
                Die::roll_for_value(6, 6, &mut dice, rng);
                value = (Die::value(&dice) as f64 / 3.0).ceil() as i32;
            }
            3 => {
                Die::roll_for_value(6, 6, &mut dice, rng);
                value = (Die::value(&dice) as f64 / 2.0).ceil() as i32;
            }
            4 => {
                Die::roll_for_value(4, 4, &mut dice, rng);
                value = Die::value(&dice);
            }
            5 => {
                Die::roll_for_value(10, 10, &mut dice, rng);
                value = Die::value(&dice);
                if value == 0 {
                    value = 10;
                }
                value = (value as f64 / 2.0).ceil() as i32;
            }
            6 => {
                Die::roll_for_value(6, 6, &mut dice, rng);
                value = Die::value(&dice);
            }
            8 => {
                Die::roll_for_value(8, 8, &mut dice, rng);
                value = Die::value(&dice);
            }
            10 => {
                Die::roll_for_value(10, 10, &mut dice, rng);
                value = Die::value(&dice);
                if value == 0 {
                    value = 10;
                }
            }
            12 => {
                Die::roll_for_value(12, 12, &mut dice, rng);
                value = Die::value(&dice);
            }
            20 => {
                Die::roll_for_value(20, 20, &mut dice, rng);
                value = Die::value(&dice);
            }
            25 => {
                Die::roll_for_value(100, 10, &mut dice, rng);
                value = Die::value(&dice);
                if value == 0 {
                    value = 100;
                }
                value = ((value as f64) / 4.0).ceil() as i32;
            }
            50 => {
                Die::roll_for_value(100, 10, &mut dice, rng);
                value = Die::value(&dice);
                if value == 0 {
                    value = 100;
                }
                value = ((value as f64) / 2.0).ceil() as i32;
            }
            100 => {
                Die::roll_for_value(100, 10, &mut dice, rng);
                value = Die::value(&dice);
                if value == 0 {
                    value = 100;
                }
            }
            1000 => {
                Die::roll_for_value(1000, 10, &mut dice, rng);
                value = Die::value(&dice);
                if value == 0 {
                    value = 1000;
                }
            }
            10000 => {
                Die::roll_for_value(10000, 10, &mut dice, rng);
                value = Die::value(&dice);
                if value == 0 {
                    value = 10000;
                }
            }
            _ => {
                if range < 10 {
                    Die::roll_for_value(range, 10, &mut dice, rng);
                    value = Die::value(&dice);
                } else if range < 20 {
                    Die::roll_for_value(range, 20, &mut dice, rng);
                    value = Die::value(&dice);
                }
            }
        }

        (dice, value)
    }

    pub fn value(dice: &Vec<Die>) -> i32 {
        let mut value = 0;
        for die in dice.into_iter() {
            match die.value {
                Some(n) => value = value + n,
                None => (),
            }
        }
        value
    }

    fn roll_for_value<R: Rng + ?Sized>(range: i32, die: i32, dice: &mut Vec<Die>, rng: &mut R) {
        if die == 10 && range % 10 == 0 {
            // percentile dice
            let mut total = 0;
            let mut n = range;
            loop {
                // 0 indexed
                let rolled = rng.random_range(0..10);

                // multiply ny next lower order
                n = n / 10;
                total = total + (rolled * n);
                dice.push(Die {
                    rolled,
                    die,
                    value: Some(rolled * n),
                });

                if n < 10 {
                    break;
                }
            }
        } else if range == die {
            // simple dice
            let rolled = rng.random_range(0..die) + 1;
            dice.push(Die {
                rolled: rolled,
                die,
                value: Some(rolled),
            });
        } else if range < die {
            // "odd" dice
            loop {
                let rolled = rng.random_range(0..die) + 1;
                if rolled > range {
                    dice.push(Die {
                        rolled,
                        die,
                        value: None,
                    })
                } else {
                    dice.insert(
                        0,
                        Die {
                            rolled,
                            die,
                            value: Some(rolled),
                        },
                    );
                    break;
                }
            }
        }
    }
}

/// Represents a single rolled value or constant in a dice expression.
///
/// A `Value` can represent either a dice roll or a constant number, along with
/// all the metadata about how it should be treated in calculations (kept/discarded,
/// bonus dice, target hits, etc.).
///
/// # Examples
///
/// ```rust
/// use dice_nom::results::Value;
/// use rand::prelude::*;
///
/// let mut rng = rand::rng();
///
/// // Create a random d6 roll
/// let roll = Value::random(6, false, &mut rng);
/// assert!(roll.sum() >= 1 && roll.sum() <= 6);
/// assert!(!roll.is_const());
///
/// // Create a constant value
/// let constant = Value::constant(5);
/// assert_eq!(constant.sum(), 5);
/// assert!(constant.is_const());
/// ```
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Value {
    /// value of this roll (or constant) before modified
    pub value: i32,

    /// range of this roll
    pub range: i32,

    /// dice that were rolled
    pub dice: Vec<Die>,

    /// modifier to the value; value + add = sum if kept == true
    add: i32,

    /// 1 by default; -1 if a "penalty" value
    mul: i32,

    /// true if this is a constant value
    constant: bool,

    /// true if this value was generated as a bonus op
    bonus: bool,

    /// true (default) if this value should be included in calculations
    keep: bool,

    /// true if this value matched a target operation
    hit: bool,

    /// the current calculated value of this roll
    sum: i32,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.keep {
            match self.bonus {
                false => {
                    if self.value != self.sum {
                        write!(f, "{} ({})", self.value, self.sum)
                    } else {
                        write!(f, "{}", self.value)
                    }
                }
                true => {
                    if self.value != self.sum {
                        write!(f, "{}* ({})", self.value, self.sum)
                    } else {
                        write!(f, "{}*", self.value)
                    }
                }
            }
        } else {
            match self.bonus {
                false => write!(f, "{}-", self.value + self.add),
                true => write!(f, "{}*-", self.value + self.add),
            }
        }
    }
}

impl Value {
    pub fn constant(value: i32) -> Value {
        Value {
            value,
            range: value,
            dice: vec![],
            add: 0,
            mul: 1,
            constant: true,
            bonus: false,
            keep: true,
            hit: false,
            sum: value,
        }
    }

    /// Helper for a common case
    pub fn d6(value: i32) -> Value {
        Value {
            value,
            range: 6,
            dice: vec![Die {
                rolled: value,
                value: Some(value),
                die: 6,
            }],
            add: 0,
            mul: 1,
            constant: false,
            bonus: false,
            keep: true,
            hit: false,
            sum: value,
        }
    }

    /// build a Value with a random number. The number rolled should mimic actual dice
    /// so the logic of rolling a random number is deferred to the Die.
    ///
    /// * Examples:
    ///
    /// ```
    /// use dice_nom::results::{ Value, Die };
    /// use rand::prelude::*;
    /// let mut rng = rand::rng();
    /// let val = Value::random(6, false, &mut rng);
    /// assert_eq!(val.range, 6);
    /// assert!(val.value >= 1);
    /// assert!(val.value <= 6);
    /// assert_eq!(val.dice.len(), 1);
    /// assert_eq!(val.dice[0].die, 6);
    /// assert!(val.dice[0].rolled >= 1);
    /// assert!(val.dice[0].rolled <= 6);
    /// assert!(val.dice[0].value.unwrap() >= 1);
    /// assert!(val.dice[0].value.unwrap() <= 6);
    /// ```
    pub fn random<R: Rng + ?Sized>(range: i32, bonus: bool, rng: &mut R) -> Value {
        let (dice, value) = Die::roll(range, rng);
        Value {
            value,
            range,
            dice: dice,
            constant: false,
            add: 0,
            mul: 1,
            bonus,
            keep: true,
            hit: true,
            sum: value,
        }
    }

    pub fn sum(&self) -> i32 {
        self.sum
    }

    pub fn is_const(&self) -> bool {
        self.constant
    }

    pub fn is_random(&self) -> bool {
        !self.is_const()
    }

    pub fn is_hit(&self) -> bool {
        self.keep && self.hit
    }

    pub fn is_bonus(&self) -> bool {
        self.bonus
    }

    pub fn is_discarded(&self) -> bool {
        !self.keep
    }

    pub fn modifier(&self) -> i32 {
        self.add
    }

    pub fn set_modifier(&mut self, add: i32) {
        self.add = add;
        if self.keep {
            self.sum = self.mul * (self.value + add);
        }
    }

    pub fn mark_bonus(&mut self) {
        self.bonus = true;
    }

    pub fn mark_penalty(&mut self) {
        self.mul = -1;
        self.sum = self.mul * (self.value + self.add);
    }

    pub fn mark_discarded(&mut self) {
        self.keep = false;
        self.sum = 0;
        self.mul = 0;
    }

    pub fn set_hit(&mut self, hit: bool) {
        self.hit = hit;
        if self.keep {
            if hit {
                self.sum = self.mul;
            } else {
                self.sum = 0;
            }
        }
    }
}

/// A collection of dice roll values that can be manipulated as a group.
///
/// A `Pool` represents the result of rolling multiple dice or combining multiple
/// values through arithmetic operations. It tracks individual values and can
/// calculate various statistics like sum, count of hits, etc.
///
/// # Examples
///
/// ```rust
/// use dice_nom::results::{Pool, Value};
///
/// let mut pool = Pool::new();
/// // Note: In practice, pools are usually created by generators
/// let values = vec![
///     Value::d6(3),
///     Value::d6(5),
///     Value::d6(1),
/// ];
/// let pool = Pool::new_with_values(values);
///
/// assert_eq!(pool.count(), 3);
/// assert_eq!(pool.sum(), 9);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Pool {
    pub values: Vec<Value>,
    total: Option<i32>,
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for v in self.values.iter() {
            if first {
                write!(f, "{}", v)?;
                first = false;
            } else {
                write!(f, ", {}", v)?;
            }
        }

        match self.total {
            Some(v) => write!(f, " = {} {{{}}}", self.sum(), v),
            None => write!(f, " = {}", self.sum()),
        }
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            values: vec![],
            total: None,
        }
    }

    pub fn new_with_values(values: Vec<Value>) -> Pool {
        Pool {
            values,
            total: None,
        }
    }

    pub fn range(&self) -> i32 {
        if self.values.is_empty() {
            0
        } else {
            self.values
                .iter()
                .filter(|&v| !v.constant)
                .map(|v| v.range)
                .max()
                .unwrap()
        }
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn sum(&self) -> i32 {
        self.values.iter().map(|v| v.sum()).sum()
    }

    pub fn kept(&self) -> usize {
        self.values.iter().filter(|v| !v.is_discarded()).count()
    }

    pub fn values(&self) -> Vec<Option<i32>> {
        self.values
            .iter()
            .map(|v| {
                if v.is_discarded() {
                    None
                } else {
                    Some(v.sum())
                }
            })
            .collect()
    }

    pub fn hits(&self) -> usize {
        self.values.iter().filter(|v| v.is_hit()).count()
    }

    pub fn bonus(&self) -> usize {
        self.values.iter().filter(|v| v.is_bonus()).count()
    }

    pub fn value(&self) -> i32 {
        if let Some(v) = self.total {
            v
        } else {
            self.sum()
        }
    }

    pub fn last_value(&self) -> i32 {
        self.values[self.count() - 1].value
    }

    pub fn last_range(&self) -> i32 {
        self.values[self.count() - 1].range
    }

    pub fn set_total(&mut self, value: i32) {
        self.total = Some(value)
    }
}

/// The final result of evaluating a dice expression.
///
/// `Results` contains the outcome of a complete dice expression evaluation,
/// including the left-hand side pool, an optional right-hand side pool
/// (for comparisons), and the final calculated value.
///
/// # Examples
///
/// ```rust
/// use dice_nom::parse;
/// use rand::prelude::*;
///
/// let mut rng = rand::rng();
/// let generator = parse("3d6+4").unwrap();
/// let results = generator.generate(&mut rng);
///
/// println!("Left side: {}", results.lhs);
/// println!("Final value: {}", results.sum());
/// // For simple expressions, rhs will be None
/// assert!(results.rhs.is_none());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Results {
    pub lhs: Pool,
    pub rhs: Option<Pool>,
    pub value: i32,
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lhs)?;
        match &self.rhs {
            Some(rhs) => write!(f, " <> {} = {}", rhs, self.sum())?,
            None => (),
        }
        write!(f, "")
    }
}

impl Results {
    pub fn sum(&self) -> i32 {
        match &self.rhs {
            Some(_) => self.value,
            None => self.lhs.value(),
        }
    }
}
