use std::fmt;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {

    /// value of this roll (or constant) before modified
    pub value: i32,

    /// range of this roll
    pub range: i32,

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
    sum: i32
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.bonus {
            false => write!(f, "{}", self.sum),
            true => write!(f, "{}*", self.sum),
        }
    }
}

impl Value {
    pub fn constant(value: i32) -> Value {
        Value{ value, range: value, add: 0, mul: 1, constant: true, bonus: false, keep: true, hit: false, sum: value }
    }

    pub fn random(range: i32, bonus: bool) -> Value {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, range + 1);
        Value{ value, range, constant: false, add: 0, mul: 1, bonus, keep: true, hit: false, sum: value }
    }

    pub fn random_with_value(value: i32, range: i32, bonus: bool) -> Value {
        Value{ value, range, constant: false, add: 0, mul: 1, bonus, keep: true, hit: false, sum: value }
    }

    pub fn sum(&self) -> i32 { self.sum }

    pub fn is_const(&self) -> bool { self.constant }

    pub fn is_random(&self) -> bool { !self.is_const() }

    pub fn is_hit(&self) -> bool { self.keep && self.hit }

    pub fn is_bonus(&self) -> bool { self.bonus }

    pub fn is_discarded(&self) -> bool { !self.keep }

    pub fn modifier(&self) -> i32 { self.add }

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

    pub fn mark_hit(&mut self) { 
        self.hit = true;
        if self.keep {
            self.sum = self.mul;
        }
    }
}

#[derive(Debug)]
pub struct Pool {
    pub values: Vec<Value>,
    sum: i32,
    value: i32,
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
        write!(f, " = {}", self.sum())
    }
}

impl Pool {
    pub fn new() -> Pool {
        Pool{ values: vec![], sum: 0, value: 0 }
    }

    pub fn new_with_values(values: Vec<Value>) -> Pool {
        let sum = values.iter().map(|&v| v.sum()).sum();
        Pool{ values, sum, value: 0 }
    }

    pub fn range(&self) -> i32 {
        if self.values.len() == 0 {
            0
        } else {
            self.values.iter().filter(|&v| !v.constant).map(|&v| v.range).max().unwrap()
        }
    }

    pub fn count(&self) -> usize { self.values.len() }

    pub fn sum(&self) -> i32 {
        self.values.iter().map(|&v| v.sum()).sum()
    }

    pub fn kept(&self) -> usize {
        self.values.iter().filter(|&v| !v.is_discarded() ).count()
    }

    pub fn hits(&self) -> usize {
        self.values.iter().filter(|&v| v.is_hit()).count()
    }

    pub fn bonus(&self) -> usize {
        self.values.iter().filter(|&v| v.is_bonus()).count()
    }

    pub fn value(&self) -> i32 { self.value }

    pub fn set_value(&mut self, value: i32) {
        self.value = value
    }
}

pub struct Results {
    pub lhs: Pool,
    pub rhs: Option<Pool>,
    pub value: i32
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lhs)?;
        match &self.rhs {
            Some(rhs) => write!(f, " <> {} = {}", rhs, self.value)?,
            None => ()
        }
        write!(f, "")
    }
}


