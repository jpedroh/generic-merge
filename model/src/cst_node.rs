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
        left: Option<&'a CSTNode<'a>>,
        right: Option<&'a CSTNode<'a>>,
    },
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
            CSTNode::Conflict { .. } => "Conflict found".into(),
        }
    }
}
