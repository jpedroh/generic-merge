use crate::ast_node::ASTNode;

#[derive(PartialEq, Debug)]
pub enum Modification {
    Unchanged,
    Added,
    Removed,
    Changed,
}

#[derive(PartialEq, Debug)]
pub struct ChangesTreeNode<'a> {
    pub kind: &'a str,
    pub identifier: &'a str,
    pub body: &'a str,
    pub modification: Modification,
    pub children: Vec<ChangesTreeNode<'a>>,
}

impl ChangesTreeNode<'_> {
    pub fn of_added(node: &ASTNode) -> ChangesTreeNode {
        return ChangesTreeNode {
            kind: &node.kind,
            identifier: &node.identifier,
            body: &node.body,
            modification: Modification::Added,
            children: node
                .children
                .iter()
                .map(ChangesTreeNode::of_added)
                .collect(),
        };
    }

    pub fn of_removed(node: &ASTNode) -> ChangesTreeNode {
        return ChangesTreeNode {
            kind: &node.kind,
            identifier: &node.identifier,
            body: &node.body,
            modification: Modification::Removed,
            children: node
                .children
                .iter()
                .map(ChangesTreeNode::of_removed)
                .collect(),
        };
    }

    pub fn of_unchanged(node: &ASTNode) -> ChangesTreeNode {
        return ChangesTreeNode {
            kind: &node.kind,
            identifier: &node.identifier,
            body: &node.body,
            modification: Modification::Unchanged,
            children: node
                .children
                .iter()
                .map(ChangesTreeNode::of_unchanged)
                .collect(),
        };
    }
}
