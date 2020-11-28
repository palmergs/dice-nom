use std::fmt;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {

    /// value of this roll (or constant) before modified
    pub value: i32,

    /// range of this roll
    pub range: i32,

    /// modifier to the value; value + add = sum if kept == true
    pub add: i32,

    /// 1 by default; -1 if a "penalty" value
    pub mul: i32,

    /// true if this is a constant value
    pub constant: bool,

    /// true if this value was generated as a bonus op
    pub bonus: bool,

    /// true (default) if this value should be included in calculations
    pub keep: bool,

    /// true if this value matched a target operation
    pub hit: bool,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.add == 0 {
            match self.bonus {
                false => write!(f, "{}", self.value),
                true => write!(f, "{}*", self.value),
            }
        } else {
            match self.bonus {
                false => write!(f, "{}{:+}", self.value, self.add),
                true => write!(f, "{}{:+}*", self.value, self.add),
            }
        }
    }
}

impl Value {
    pub fn constant(value: i32) -> Value {
        Value{ value, range: value, add: 0, mul: 1, constant: true, bonus: false, keep: true, hit: false }
    }

    pub fn random(range: i32, bonus: bool) -> Value {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, range + 1);
        Value{ value, range, constant: false, add: 0, mul: 1, bonus, keep: true, hit: false }
    }

    pub fn sum(&self) -> i32 {
        if self.keep {
            self.value + self.add
        } else {
            0
        }
    }

    pub fn is_const(&self) -> bool {
        self.constant
    }

    pub fn is_random(&self) -> bool {
        !self.is_const()
    }

    pub fn hit(&self) -> i32 {
        if self.keep && self.hit {
            self.mul as i32
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct Pool {
    pub values: Vec<Value>,
    pub value: i32,
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
        write!(f, "")
    }
}

impl Pool {
    pub fn new() -> Pool {
        Pool{ values: vec![], value: 0 }
    }

    pub fn range(&self) -> i32 {
        if self.values.len() == 0 {
            0
        } else {
            self.values.iter().filter(|&v| !v.constant).map(|&v| v.range).max().unwrap()
        }
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn constants(&self) -> usize {
        self.values.iter().filter(|&v| v.constant ).count()
    }

    pub fn kept(&self) -> usize {
        self.values.iter().filter(|&v| v.keep ).count()
    }

    pub fn bonus(&self) -> usize {  
        self.values.iter().filter(|&v| v.bonus ).count()
    }

    pub fn sum(&self) -> i32 {
        self.values.iter().map(|&v| v.sum() ).sum()
    }

    pub fn hits(&self) -> i32 {
        self.values.iter().map(|&v| v.hit() ).sum()
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


