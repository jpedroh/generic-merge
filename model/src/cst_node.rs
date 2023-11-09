#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum CSTNode<'a> {
    Terminal {
        kind: &'a str,
        value: String,
        start_position: tree_sitter::Point,
        end_position: tree_sitter::Point,
    },
    NonTerminal {
        kind: &'a str,
        children: Vec<CSTNode<'a>>,
        start_position: tree_sitter::Point,
        end_position: tree_sitter::Point,
    },
    Conflict {
        left: Option<&'a CSTNode<'a>>,
        right: Option<&'a CSTNode<'a>>,
    },
}

impl CSTNode<'_> {
    pub fn can_be_matching_unordered(&self) -> bool {
        match self {
            CSTNode::Terminal { .. } => false,
            CSTNode::NonTerminal { kind, .. } => vec!["interface_body"].contains(kind),
            CSTNode::Conflict { .. } => false,
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
                    result.push_str(" ");
                    result.push_str(&current.clone().to_string());
                    result
                })
            }
            CSTNode::Conflict { left, right } => match (left, right) {
                (Some(left), Some(right)) => format!(
                    "<<<<<<<<< {} ========= {} >>>>>>>>>",
                    left.to_string(),
                    right.to_string()
                )
                .to_string(),
                (Some(left), None) => {
                    format!("<<<<<<<<< ========= {} >>>>>>>>>", left.to_string()).to_string()
                }
                (None, Some(right)) => {
                    format!("<<<<<<<<< ========= {} >>>>>>>>>", right.to_string()).to_string()
                }
                (None, None) => panic!("Invalid conflict provided"),
            },
        }
    }
}
