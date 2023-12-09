use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum AoCResult {
    None,
    String(String),
    Int(i64),
    UInt(u64),
    USize(usize),
    BigInt(i128),
    BigUInt(u128),
    Float(f64),
}

impl Display for AoCResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoCResult::None => write!(f, "Empty"),
            AoCResult::String(s) => write!(f, "{}", s),
            AoCResult::Int(i) => write!(f, "{}", i),
            AoCResult::UInt(i) => write!(f, "{}", i),
            AoCResult::USize(i) => write!(f, "{}", i),
            AoCResult::BigInt(i) => write!(f, "{}", i),
            AoCResult::BigUInt(i) => write!(f, "{}", i),
            AoCResult::Float(i) => write!(f, "{}", i),
        }
    }
}

impl From<String> for AoCResult {
    fn from(val: String) -> Self {
        AoCResult::String(val)
    }
}

impl From<&str> for AoCResult {
    fn from(val: &str) -> Self {
        AoCResult::String(val.into())
    }
}

impl From<i64> for AoCResult {
    fn from(val: i64) -> Self {
        AoCResult::Int(val)
    }
}

impl From<u64> for AoCResult {
    fn from(val: u64) -> Self {
        AoCResult::UInt(val)
    }
}

impl From<usize> for AoCResult {
    fn from(val: usize) -> Self {
        AoCResult::USize(val)
    }
}

impl From<i128> for AoCResult {
    fn from(val: i128) -> Self {
        AoCResult::BigInt(val)
    }
}

impl From<u128> for AoCResult {
    fn from(val: u128) -> Self {
        AoCResult::BigUInt(val)
    }
}

impl From<f64> for AoCResult {
    fn from(val: f64) -> Self {
        AoCResult::Float(val)
    }
}

impl From<()> for AoCResult {
    fn from(_val: ()) -> Self {
        AoCResult::None
    }
}
