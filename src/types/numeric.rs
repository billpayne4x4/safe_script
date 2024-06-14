#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Numeric {
    Int(i64),
    Float(f64),
}