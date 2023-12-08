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

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct NonTerminal<'a> {
    pub kind: &'a str,
    pub children: Vec<CSTNode<'a>>,
    pub start_position: Point,
    pub end_position: Point,
    pub are_children_unordered: bool,
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Terminal<'a> {
    pub kind: &'a str,
    pub value: &'a str,
    pub start_position: Point,
    pub end_position: Point,
}
