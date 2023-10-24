use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

use rust_decimal::Decimal;

pub type Int = i64;
pub type Float = Decimal;

#[derive(Debug, Clone, Copy)]
pub enum Data {
    Int(Int),
    Float(Float),
}

impl Add<Data> for Data {
    type Output = Option<Data>;
    fn add(self, rhs: Data) -> Self::Output {
        if let Data::Int(a) = self {
            if let Data::Int(b) = rhs {
                Some(Data::Int(a + b))
            } else {
                None
            }
        } else if let Data::Float(a) = self {
            if let Data::Float(b) = rhs {
                Some(Data::Float(a + b))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Sub<Data> for Data {
    type Output = Option<Data>;
    fn sub(self, rhs: Data) -> Self::Output {
        match self {
            Data::Int(a) => match rhs {
                Data::Int(b) => Some(Data::Int(a - b)),
                _ => None,
            },
            Data::Float(a) => match rhs {
                Data::Float(b) => Some(Data::Float(a - b)),
                _ => None,
            },
        }
    }
}

impl Mul<Data> for Data {
    type Output = Option<Data>;
    fn mul(self, rhs: Data) -> Self::Output {
        match self {
            Data::Int(a) => match rhs {
                Data::Int(b) => Some(Data::Int(a * b)),
                _ => None,
            },
            Data::Float(a) => match rhs {
                Data::Float(b) => Some(Data::Float(a * b)),
                _ => None,
            },
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Data::Int(a) => match other {
                Data::Int(b) => a == b,
                Data::Float(b) => Float::new(*a, 0) == *b,
            }
            Data::Float(a) => match other {
                Data::Int(b) => Float::new(*b, 0) == *a,
                Data::Float(b) => a == b,
            }
        }
    }
}

impl Eq for Data {}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Data::Int(a) => match other {
                Data::Int(b) => Some(a.cmp(b)),
                Data::Float(b) => Some(Float::new(*a, 0).cmp(b)),
            }
            Data::Float(a) => match other {
                Data::Int(b) => Some(a.cmp(&Float::new(*b, 0))),
                Data::Float(b) => Some(a.cmp(b)),
            }
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Float(a) => write!(f, "{a}"),
            Data::Int(a) => write!(f, "{a}"),
        }
    }
}
