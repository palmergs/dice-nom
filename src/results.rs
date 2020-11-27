use std::fmt;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub add: i32,
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
        Value{ value, min: value, max: value, add: 0, bonus: false, keep: true, sum: value }
    }

    pub fn random(range: i32, bonus: bool) -> Value {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, range + 1);
        Value{ value, min: range, max: range, add: 0, bonus, keep: true, sum: value }
    }

    pub fn is_const(&self) -> bool {
        self.min == self.max
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

    pub fn add_const(&mut self, value: i32) {
        self.values.push(Value::constant(value));
    }

    pub fn add_random(&mut self, range: i32, bonus: bool) {
        let r = Value::random(range, bonus);
        self.values.push(Value::random(range, bonus));
        self.kept += 1;
        self.sum += r.sum;
        if bonus {
            self.bonus += 1;
        }
    }
}

