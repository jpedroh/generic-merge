use model::CSTNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matching<'a> {
    pub matching_node: &'a CSTNode,
    pub score: usize,
}
