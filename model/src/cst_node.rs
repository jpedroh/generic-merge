use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, _hasher: &mut H) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.row.hash(&mut hasher);
        self.column.hash(&mut hasher);
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum CSTNode<'a> {
    Terminal(Terminal<'a>),
    NonTerminal(NonTerminal<'a>),
}

impl<'a> Hash for CSTNode<'a> {
    fn hash<H: Hasher>(&self, _hasher: &mut H) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        match self {
            CSTNode::Terminal(node) => {
                node.hash(&mut hasher);
            }
            CSTNode::NonTerminal(node) => {
                node.hash(&mut hasher);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct NonTerminal<'a> {
    pub kind: &'a str,
    pub children: Vec<CSTNode<'a>>,
    pub start_position: Point,
    pub end_position: Point,
}

impl<'a> Hash for NonTerminal<'a> {
    fn hash<H: Hasher>(&self, _hasher: &mut H) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        self.kind.hash(&mut hasher);
        self.children.hash(&mut hasher);
        self.start_position.hash(&mut hasher);
        self.end_position.hash(&mut hasher);
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Terminal<'a> {
    pub kind: &'a str,
    pub value: &'a str,
    pub start_position: Point,
    pub end_position: Point,
}

impl<'a> Hash for Terminal<'a> {
    fn hash<H: Hasher>(&self, _hasher: &mut H) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        self.kind.hash(&mut hasher);
        self.value.hash(&mut hasher);
        self.start_position.hash(&mut hasher);
        self.end_position.hash(&mut hasher);
    }
}

impl NonTerminal<'_> {
    pub fn are_children_unordered(&self) -> bool {
        ["interface_body", "class_body"].contains(&self.kind)
    }
}
