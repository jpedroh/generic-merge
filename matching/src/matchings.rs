use std::collections::HashMap;

use model::CSTNode;
use utils::unordered_pair::UnorderedPair;

use crate::matching::Matching;
use crate::matching_entry::MatchingEntry;

#[derive(Debug, Clone)]
pub struct Matchings<'a> {
    matching_entries: HashMap<UnorderedPair<CSTNode<'a>>, MatchingEntry>,
}

impl<'a> Matchings<'a> {
    pub fn empty() -> Self {
        Matchings {
            matching_entries: HashMap::new(),
        }
    }

    pub fn new(matching_entries: HashMap<UnorderedPair<CSTNode<'a>>, MatchingEntry>) -> Self {
        Matchings { matching_entries }
    }

    pub fn find_matching_for(&self, a_node: &'a CSTNode) -> Option<Matching> {
        self.matching_entries
            .iter()
            .find(|(UnorderedPair(left, right), ..)| left == a_node || right == a_node)
            .map(|(UnorderedPair(left, right), matching)| {
                let matching_node = if left == a_node { right } else { left };
                Matching {
                    matching_node,
                    score: matching.score,
                    is_perfect_match: matching.is_perfect_match,
                }
            })
    }

    pub fn get_matching_entry(
        &'a self,
        left: CSTNode<'a>,
        right: CSTNode<'a>,
    ) -> Option<&MatchingEntry> {
        self.matching_entries.get(&UnorderedPair(left, right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_if_a_matching_for_the_node_is_not_found() {
        let a_node = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };

        assert_eq!(None, Matchings::empty().find_matching_for(&a_node))
    }

    #[test]
    fn returns_some_match_if_a_matching_for_the_node_is_found() {
        let a_node = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };

        let mut matchings = HashMap::new();
        matchings.insert(
            UnorderedPair(a_node.clone(), a_node.clone()),
            MatchingEntry::new(1, true),
        );

        assert_eq!(
            Some(Matching {
                matching_node: &a_node,
                score: 1,
                is_perfect_match: true
            }),
            Matchings::new(matchings).find_matching_for(&a_node)
        )
    }
}
