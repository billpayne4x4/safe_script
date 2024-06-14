use crate::types::tokens::Tokens;
use crate::types::dynamic_value::DynamicValue;

pub struct AstNode {
    pub token: Tokens,
    pub left: Option<Box<DynamicValue>>,
    pub right: Option<Box<DynamicValue>>,
}

impl AstNode {
    pub fn new(token: Tokens, left: Option<DynamicValue>, right: Option<DynamicValue>) -> Self {
        AstNode {
            token,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }
}