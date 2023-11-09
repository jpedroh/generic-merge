use model::CSTNode;

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum MergedCSTNode<'a> {
    Terminal {
        kind: &'a str,
        value: String,
    },
    NonTerminal {
        kind: &'a str,
        children: Vec<MergedCSTNode<'a>>,
    },
    Conflict {
        left: Option<Box<MergedCSTNode<'a>>>,
        right: Option<Box<MergedCSTNode<'a>>>,
    },
}

impl<'a> Into<MergedCSTNode<'a>> for CSTNode<'a> {
    fn into(self) -> MergedCSTNode<'a> {
        match self {
            CSTNode::Terminal { kind, value, .. } => MergedCSTNode::Terminal { kind, value },
            CSTNode::NonTerminal { kind, children, .. } => MergedCSTNode::NonTerminal {
                kind,
                children: children.into_iter().map(|node| node.into()).collect(),
            },
        }
    }
}

impl ToString for MergedCSTNode<'_> {
    fn to_string(&self) -> String {
        match self {
            MergedCSTNode::Terminal { value, .. } => value.to_string(),
            MergedCSTNode::NonTerminal { children, .. } => {
                children.iter().fold(String::new(), |acc, current| {
                    let mut result = acc.to_owned();
                    result.push_str(" ");
                    result.push_str(&current.clone().to_string());
                    result
                })
            }
            MergedCSTNode::Conflict { left, right } => match (left, right) {
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
