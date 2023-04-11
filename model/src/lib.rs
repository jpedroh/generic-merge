#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum CSTNode {
    Terminal {
        kind: String,
        value: String,
    },
    NonTerminal {
        kind: String,
        children: Vec<CSTNode>,
    },
}
