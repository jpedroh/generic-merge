use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.row.hash(&mut hasher);
        self.column.hash(&mut hasher);
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum CSTNode<'a> {
    Terminal {
        kind: &'a str,
        value: &'a str,
        start_position: Point,
        end_position: Point,
    },
    NonTerminal {
        kind: &'a str,
        children: Vec<CSTNode<'a>>,
        start_position: Point,
        end_position: Point,
    },
}

impl<'a> Hash for CSTNode<'a> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        match self {
            CSTNode::Terminal {
                kind,
                value,
                start_position,
                end_position,
            } => {
                kind.hash(&mut hasher);
                value.hash(&mut hasher);
                start_position.hash(&mut hasher);
                end_position.hash(&mut hasher);
            }
            CSTNode::NonTerminal {
                kind,
                children,
                start_position,
                end_position,
            } => {
                kind.hash(&mut hasher);
                children.hash(&mut hasher);
                start_position.hash(&mut hasher);
                end_position.hash(&mut hasher);
            }
        }
    }
}

impl CSTNode<'_> {
    pub fn are_children_unordered(&self) -> bool {
        match self {
            CSTNode::Terminal { .. } => false,
            CSTNode::NonTerminal { kind, .. } => ["interface_body", "class_body"].contains(kind),
        }
    }
}

impl ToString for CSTNode<'_> {
    fn to_string(&self) -> String {
        match self {
            CSTNode::Terminal { value, .. } => value.to_string(),
            CSTNode::NonTerminal { children, .. } => {
                children.iter().fold(String::new(), |acc, current| {
                    let mut result = acc.to_owned();
                    result.push(' ');
                    result.push_str(&current.clone().to_string());
                    result
                })
            }
        }
    }
}
