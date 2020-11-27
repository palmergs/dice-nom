use std::fmt;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {
    pub value: i32,
    pub range: i32,
    pub add: i32,
    pub constant: bool,
    pub bonus: bool,
    pub keep: bool,

    pub sum: i32
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
        Value{ value, range: value, add: 0, constant: true, bonus: false, keep: true, sum: value }
    }

    pub fn random(range: i32, bonus: bool) -> Value {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, range + 1);
        Value{ value, range, constant: false, add: 0, bonus, keep: true, sum: value }
    }

    pub fn is_const(&self) -> bool {
        self.constant
    }

    pub fn is_random(&self) -> bool {
        !self.is_const()
    }
}

pub struct Pool {
    pub values: Vec<Value>,

    pub kept: i32,
    pub bonus: i32,
    pub sum: i32
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
        Pool{ values: vec![], kept: 0, bonus: 0, sum: 0 }
    }
}

