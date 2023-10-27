use model::CSTNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matching<'a> {
    pub matching_node: &'a CSTNode<'a>,
    pub score: usize,
    pub is_perfect_match: bool,
}
