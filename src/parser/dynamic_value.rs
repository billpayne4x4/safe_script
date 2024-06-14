use std::collections::HashMap;
use crate::parser::ast_node::AstNode;

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

pub(crate) enum DynamicValue {
    Node(Box<AstNode>),
    Int(i64),
    Long(f64),
    String(String),
    Bool(bool),
    Char(char),
    Byte(u8),
    Array(Vec<DynamicValue>),
    Object(HashMap<String, DynamicValue>),
    Null,
}

impl DynamicValue {
    pub(crate) fn as_node(&self) -> Option<&AstNode> {
        match self {
            DynamicValue::Node(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_int(&self) -> Option<i64> {
        match self {
            DynamicValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub(crate) fn as_long(&self) -> Option<f64> {
        match self {
            DynamicValue::Long(f) => Some(*f),
            _ => None,
        }
    }

    pub(crate) fn as_string(&self) -> Option<&str> {
        match self {
            DynamicValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub(crate) fn as_bool(&self) -> Option<bool> {
        match self {
            DynamicValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub(crate) fn as_char(&self) -> Option<char> {
        match self {
            DynamicValue::Char(c) => Some(*c),
            _ => None,
        }
    }

    pub(crate) fn as_byte(&self) -> Option<u8> {
        match self {
            DynamicValue::Byte(b) => Some(*b),
            _ => None,
        }
    }

    pub(crate) fn as_array(&self) -> Option<&Vec<DynamicValue>> {
        match self {
            DynamicValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub(crate) fn as_object(&self) -> Option<&HashMap<String, DynamicValue>> {
        match self {
            DynamicValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub(crate) fn as_null(&self) -> bool {
        match self {
            DynamicValue::Null => true,
            _ => false,
        }
    }

    pub(crate) fn get_type(&self) -> VarType {
        match self {
            DynamicValue::Node(_) => VarType::Node,
            DynamicValue::Int(_) => VarType::Int,
            DynamicValue::Long(_) => VarType::Float,
            DynamicValue::String(_) => VarType::String,
            DynamicValue::Bool(_) => VarType::Bool,
            DynamicValue::Char(_) => VarType::Char,
            DynamicValue::Byte(_) => VarType::Byte,
            DynamicValue::Array(_) => VarType::Array,
            DynamicValue::Object(_) => VarType::Object,
            DynamicValue::Null => VarType::Null,
        }
    }
}