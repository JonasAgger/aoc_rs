use std::fmt::Display;



#[derive(Debug, PartialEq, PartialOrd)]
pub enum AoCResult {
    None,
    String(String),
    Int(i64),
    BigInt(i128),
    Float(f64),
}

impl Display for AoCResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AoCResult::None => write!(f, "Empty"),
            AoCResult::String(s) => write!(f, "{}", s),
            AoCResult::Int(i) => write!(f, "{}", i),
            AoCResult::BigInt(i) => write!(f, "{}", i),
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

impl Into<AoCResult> for i128 {
    fn into(self) -> AoCResult {
        AoCResult::BigInt(self)
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