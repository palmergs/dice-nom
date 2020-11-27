use std::fmt;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {
    pub value: i32,
    pub range: i32,
    pub add: i32,
    pub constant: bool,
    pub bonus: bool,
    pub keep: bool
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
        Value{ value, range: value, add: 0, constant: true, bonus: false, keep: true }
    }

    pub fn random(range: i32, bonus: bool) -> Value {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, range + 1);
        Value{ value, range, constant: false, add: 0, bonus, keep: true }
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
}

#[derive(Debug)]
pub struct Pool {
    pub values: Vec<Value>,
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
        Pool{ values: vec![] }
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
}


