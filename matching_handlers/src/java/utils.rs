use model::{cst_node::Terminal, CSTNode};

pub fn find_identifier<'a>(node_children: &'a [CSTNode<'a>]) -> Option<&'a Terminal<'a>> {
    node_children
        .iter()
        .find(|node| node.kind() == "identifier")
        .and_then(|node| match node {
            CSTNode::Terminal(terminal) => Some(terminal),
            CSTNode::NonTerminal(_) => None,
        })
}
