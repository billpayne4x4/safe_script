use crate::parser::ast_node::AstNode;

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