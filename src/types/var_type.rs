#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VarType {
    Node,
    Int,
    Float,
    String,
    Bool,
    Char,
    Byte,
    Array,
    Object,
    Null,
}

impl VarType {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "int" => Some(VarType::Int),
            "float" => Some(VarType::Float),
            "string" => Some(VarType::String),
            "bool" => Some(VarType::Bool),
            "char" => Some(VarType::Char),
            "byte" => Some(VarType::Byte),
            "array" => Some(VarType::Array),
            "null" => Some(VarType::Null),
            _ => None,
        }
    }
}