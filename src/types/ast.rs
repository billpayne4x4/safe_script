use crate::types::ast_node::AstNode;

pub struct Ast {
    pub statements: Vec<AstNode>,
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            statements: Vec::new(),
        }
    }
}