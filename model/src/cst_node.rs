#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum CSTNode<'a> {
    Terminal {
        kind: &'a str,
        value: String,
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

impl CSTNode<'_> {
    pub fn are_children_unordered(&self) -> bool {
        match self {
            CSTNode::Terminal { .. } => false,
            CSTNode::NonTerminal { kind, .. } => ["interface_body"].contains(kind),
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
