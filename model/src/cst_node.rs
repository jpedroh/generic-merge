#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum CSTNode<'a> {
    Terminal {
        kind: &'a str,
        value: String,
    },
    NonTerminal {
        kind: &'a str,
        children: Vec<CSTNode<'a>>,
    },
    Conflict {
        left: Box<Option<CSTNode<'a>>>,
        right: Box<Option<CSTNode<'a>>>,
    },
}
