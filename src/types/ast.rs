use crate::types::ast_node::AstNode;

pub(crate) struct Ast {
    pub statements: Vec<AstNode>,
}

impl Ast {
    pub(crate) fn new() -> Self {
        Ast {
            statements: Vec::new(),
        }
    }
}