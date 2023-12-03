
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

impl Into<AoCResult> for String {
    fn into(self) -> AoCResult {
        AoCResult::String(self)
    }
}

impl Into<AoCResult> for &str {
    fn into(self) -> AoCResult {
        AoCResult::String(self.into())
    }
}

impl Into<AoCResult> for i64 {
    fn into(self) -> AoCResult {
        AoCResult::Int(self)
    }
}

impl Into<AoCResult> for u64 {
    fn into(self) -> AoCResult {
        AoCResult::UInt(self)
    }
}

impl Into<AoCResult> for usize {
    fn into(self) -> AoCResult {
        AoCResult::USize(self)
    }
}

impl Into<AoCResult> for i128 {
    fn into(self) -> AoCResult {
        AoCResult::BigInt(self)
    }
}

impl Into<AoCResult> for u128 {
    fn into(self) -> AoCResult {
        AoCResult::BigUInt(self)
    }
}

impl Into<AoCResult> for f64 {
    fn into(self) -> AoCResult {
        AoCResult::Float(self)
    }
}

impl Into<AoCResult> for () {
    fn into(self) -> AoCResult {
        AoCResult::None
    }
}