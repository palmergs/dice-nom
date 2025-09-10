use rand::Rng;
use serde::Serialize;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub struct Die {
    /// value on the physical die
    pub rolled: i32,

    /// range of the physical die
    pub range: i32,

    /// calculated value
    pub value: i32,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.rolled, self.range)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DiscardOp {
    Highest(i32),
    Lowest(i32),
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
    /// let mut rng = rand::thread_rng();
    /// let dice = Die::roll(1000, None, &mut rng);
    /// assert!(dice.is_some());
    /// assert_eq!(dice.as_ref().unwrap().len(), 3);
    /// let value = dice.unwrap().iter().map(|d| d.value).sum::<i32>();
    /// assert!(value >= 0);
    /// assert!(value < 1000);
    /// ```
    pub fn roll<R: Rng + ?Sized>(
        range: i32,
        discard: Option<DiscardOp>,
        rng: &mut R,
    ) -> Option<Vec<Die>> {
        let mut accum = Vec::new();
        match range {
            2 => Die::roll_with_discard(2, 6, discard, &mut accum, rng),
            3 => Die::roll_with_discard(3, 6, discard, &mut accum, rng),
            4 => Die::roll_with_discard(4, 4, discard, &mut accum, rng),
            5 => Die::roll_with_discard(5, 10, discard, &mut accum, rng),
            6 => Die::roll_with_discard(6, 6, discard, &mut accum, rng),
            8 => Die::roll_with_discard(8, 8, discard, &mut accum, rng),
            10 => Die::roll_with_discard(10, 10, discard, &mut accum, rng),
            12 => Die::roll_with_discard(12, 12, discard, &mut accum, rng),
            20 => Die::roll_with_discard(20, 20, discard, &mut accum, rng),
            100 => {
                Die::roll_with_discard(100, 100, discard, &mut accum, rng);
                Die::roll_with_discard(10, 10, discard, &mut accum, rng);
            }
            1000 => {
                Die::roll_with_discard(1000, 1000, discard, &mut accum, rng);
                Die::roll_with_discard(100, 100, discard, &mut accum, rng);
                Die::roll_with_discard(10, 10, discard, &mut accum, rng);
            }
            10000 => {
                Die::roll_with_discard(10000, 10000, discard, &mut accum, rng);
                Die::roll_with_discard(1000, 1000, discard, &mut accum, rng);
                Die::roll_with_discard(100, 100, discard, &mut accum, rng);
                Die::roll_with_discard(10, 10, discard, &mut accum, rng);
            }
            _ => {
                if range < 10 {
                    Die::roll_with_discard(range, 10, discard, &mut accum, rng)
                } else if range < 20 {
                    Die::roll_with_discard(range, 20, discard, &mut accum, rng)
                }
            }
        }

        if accum.len() > 0 { Some(accum) } else { None }
    }

    fn roll_with_discard<R: Rng + ?Sized>(
        range: i32,
        die_range: i32,
        discard: Option<DiscardOp>,
        dice: &mut Vec<Die>,
        rng: &mut R,
    ) {
        let mut accum = Vec::new();
        Die::roll_until_success(range, die_range, &mut accum, rng);
        let mut last_idx = accum.len() - 1;
        match discard {
            Some(DiscardOp::Highest(n)) => {
                let mut min = accum[0].0;
                for _ in 0..n {
                    Die::roll_until_success(range, die_range, &mut accum, rng);
                    let curr_idx = accum.len() - 1;
                    if accum[accum.len() - 1].0 < min {
                        min = accum[accum.len() - 1].0;
                        accum[last_idx].1 = None;
                        last_idx = curr_idx;
                    } else {
                        accum[curr_idx].1 = None;
                    }
                }
            }
            Some(DiscardOp::Lowest(n)) => {
                let mut max = accum[0].0;
                for _ in 0..n {
                    Die::roll_until_success(range, die_range, &mut accum, rng);
                    let curr_idx = accum.len() - 1;
                    if accum[accum.len() - 1].0 > max {
                        max = accum[accum.len() - 1].0;
                        accum[last_idx].1 = None;
                        last_idx = curr_idx;
                    } else {
                        accum[curr_idx].1 = None;
                    }
                }
            }
            _ => {}
        }

        for die in accum {
            match die {
                (rolled, Some(value)) => dice.push(Die {
                    rolled,
                    range,
                    value,
                }),
                (rolled, None) => dice.push(Die {
                    rolled,
                    range,
                    value: 0,
                }),
            }
        }
    }

    fn roll_until_success<R: Rng + ?Sized>(
        range: i32,
        die_range: i32,
        accum: &mut Vec<(i32, Option<i32>)>,
        rng: &mut R,
    ) {
        loop {
            let result = Die::roll_in_range(range, die_range, rng);
            accum.push(result);
            if result.1.is_some() {
                break;
            }
        }
    }

    fn roll_in_range<R: Rng + ?Sized>(
        range: i32,
        die_range: i32,
        rng: &mut R,
    ) -> (i32, Option<i32>) {
        let value = Die::roll_one(die_range, rng);
        if value > range {
            (value, None)
        } else {
            (value, Some(value))
        }
    }

    fn roll_one<R: Rng + ?Sized>(range: i32, rng: &mut R) -> i32 {
        match range {
            10000 => rng.gen_range(0..10) * 1000,
            1000 => rng.gen_range(0..10) * 100,
            100 => rng.gen_range(0..10) * 10,
            10 => rng.gen_range(0..10),
            _ => rng.gen_range(0..range) + 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Value {
    /// value of this roll (or constant) before modified
    pub value: i32,

    /// range of this roll
    pub range: i32,

    /// dice that were rolled
    pub dice: Option<Vec<Die>>,

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
            dice: None,
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
            dice: Some(vec![Die {
                rolled: value,
                value,
                range: 6,
            }]),
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
    /// let mut rng = rand::thread_rng();
    /// let val = Value::random(6, false, &mut rng);
    /// assert_eq!(val.range, 6);
    /// assert!(val.value >= 1);
    /// assert!(val.value <= 6);
    /// assert!(val.dice.is_some());
    /// assert_eq!(val.dice.as_ref().unwrap().len(), 1);
    /// assert_eq!(val.dice.as_ref().unwrap()[0].range, 6);
    /// assert!(val.dice.as_ref().unwrap()[0].rolled >= 1);
    /// assert!(val.dice.as_ref().unwrap()[0].rolled <= 6);
    /// assert!(val.dice.as_ref().unwrap()[0].value >= 1);
    /// assert!(val.dice.as_ref().unwrap()[0].value <= 6);
    /// ```
    pub fn random<R: Rng + ?Sized>(range: i32, bonus: bool, rng: &mut R) -> Value {
        let dice = Die::roll(range, None, rng).unwrap_or(vec![]);
        let value = dice.iter().map(|d| d.value).sum::<i32>();
        Value {
            value,
            range,
            dice: Some(dice),
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

#[derive(Debug, Serialize)]
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

#[derive(Serialize)]
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
