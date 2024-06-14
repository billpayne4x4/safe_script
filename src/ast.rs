pub(crate) struct Ast {
    pub nodes: Vec<AstNode>,
}

impl Ast {
    pub(crate) fn new() -> Self {
        Ast {
            nodes: Vec::new(),
        }
    }
}