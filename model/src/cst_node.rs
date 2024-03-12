use std::hash::Hash;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
pub enum CSTNode<'a> {
    Terminal(Terminal<'a>),
    NonTerminal(NonTerminal<'a>),
}

impl CSTNode<'_> {
    pub fn id(&self) -> uuid::Uuid {
        match self {
            CSTNode::Terminal(terminal) => terminal.id,
            CSTNode::NonTerminal(non_terminal) => non_terminal.id,
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            CSTNode::Terminal(terminal) => terminal.kind,
            CSTNode::NonTerminal(non_terminal) => non_terminal.kind,
        }
    }

    pub fn contents(&self) -> String {
        match self {
            CSTNode::Terminal(node) => node.to_string(),
            CSTNode::NonTerminal(node) => node.to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct NonTerminal<'a> {
    pub id: uuid::Uuid,
    pub kind: &'a str,
    pub children: Vec<CSTNode<'a>>,
    pub start_position: Point,
    pub end_position: Point,
    pub are_children_unordered: bool,
}

impl<'a> TryFrom<&'a CSTNode<'a>> for &'a NonTerminal<'a> {
    type Error = &'static str;

    fn try_from(node: &'a CSTNode<'a>) -> Result<Self, Self::Error> {
        match node {
            CSTNode::NonTerminal(non_terminal) => Ok(non_terminal),
            CSTNode::Terminal(_) => Err("Cannot convert terminal to non-terminal"),
        }
    }
}

impl ToString for NonTerminal<'_> {
    fn to_string(&self) -> String {
        self.children.iter().fold(String::from(""), |acc, node| {
            format!("{} {}", acc, node.contents())
        })
    }
}

#[derive(Debug, Default, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Terminal<'a> {
    pub id: uuid::Uuid,
    pub kind: &'a str,
    pub value: &'a str,
    pub start_position: Point,
    pub end_position: Point,
    pub is_block_end_delimiter: bool,
}

impl ToString for Terminal<'_> {
    fn to_string(&self) -> String {
        String::from(self.value)
    }
}
