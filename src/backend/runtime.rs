use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ExprValue {
    None,
    Bool(bool),
    Int(i32),
    Float(f64),
}

impl fmt::Display for ExprValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ExprValue::None => format!("()"),
                ExprValue::Int(ref ival) => format!("{}", ival),
                ExprValue::Float(ref fval) => format!("{}", fval),
                ExprValue::Bool(bval) => format!("{}", bval),
            }
        )
    }
}

impl Add for ExprValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs + rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl Sub for ExprValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs - rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl Mul for ExprValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs * rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl Div for ExprValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs / rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl Rem for ExprValue {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs % rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl BitAnd for ExprValue {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs & rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl BitOr for ExprValue {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs | rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl BitXor for ExprValue {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs ^ rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl Shl for ExprValue {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs << rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl Shr for ExprValue {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        match self {
            ExprValue::Int(ref lhs) => match rhs {
                ExprValue::Int(ref rhs) => ExprValue::Int(lhs >> rhs),
                _ => ExprValue::None,
            },
            _ => ExprValue::None,
        }
    }
}

impl Neg for ExprValue {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            ExprValue::Int(ref val) => ExprValue::Int(-val),
            _ => ExprValue::None,
        }
    }
}

impl Not for ExprValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ExprValue::Int(ival) => ExprValue::Int(!ival),
            ExprValue::Bool(bval) => ExprValue::Bool(!bval),
            _ => ExprValue::None,
        }
    }
}

pub struct Runtime {
    bindings: HashMap<isize, HashMap<String, ExprValue>>,
    level: isize,
}

impl Runtime {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(0isize, HashMap::new());
        Runtime { bindings, level: 0 }
    }

    pub fn save_binding(&mut self, id: &str, value: ExprValue) {
        self.bindings
            .get_mut(&self.level)
            .unwrap()
            .insert(id.to_owned(), value);
    }

    pub fn get_binding(&self, check_id: &str) -> ExprValue {
        let mut level = self.level;

        while level >= 0 {
            for (id, val) in self.bindings.get(&level).unwrap().iter() {
                if check_id == *id {
                    return val.clone();
                }
            }

            level -= 1;
        }
        ExprValue::None
    }

    pub fn open_level(&mut self) {
        self.level += 1;
        self.bindings.insert(self.level, HashMap::new());
    }

    pub fn close_level(&mut self) {
        self.bindings.remove(&self.level);
        self.level -= 1;
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut level = self.level;

        while level >= 0 {
            println!("Bindings for level {}...", level);
            for (id, val) in self.bindings.get(&level).unwrap().iter() {
                let _ = write!(f, "{:?} => {:?}", id, val);
            }
            level -= 1;
        }
        write!(f, "\n")
    }
}
