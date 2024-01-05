use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};
use unordered_pair::UnorderedPair;

use crate::{calculate_matchings, MatchingEntry, Matchings};

pub fn unordered_tree_matching<'a>(left: &'a CSTNode, right: &'a CSTNode) -> crate::Matchings<'a> {
    match (left, right) {
        (
            CSTNode::Terminal(Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            CSTNode::Terminal(Terminal {
                kind: kind_right,
                value: value_right,
                ..
            }),
        ) => {
            let is_perfetch_match = kind_left == kind_right && value_left == value_right;
            Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(is_perfetch_match.into(), is_perfetch_match),
            )
        }
        (
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_left,
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_right,
                children: children_right,
                ..
            }),
        ) => {
            let root_matching: usize = (kind_left == kind_right).into();

            let mut sum = 0;
            let mut result = Matchings::empty();

            for child_left in children_left {
                for child_right in children_right {
                    let matching_score = if child_left.kind() == "field_declaration" && child_left.kind() == child_right.kind() {
                        compute_matching_score_for_field_declaration(child_left, child_right)
                    } else {
                        compute_matching_score(child_left, child_right)
                    };

                    if matching_score == 1 {
                        let child_matching = calculate_matchings(child_left, child_right);
                        sum += child_matching
                            .get_matching_entry(child_left, child_right)
                            .map_or(0, |matching| matching.score);
                        result.extend(child_matching);
                    }
                }
            }

            result.extend(Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry {
                    score: sum + root_matching,
                    is_perfect_match: left.contents() == right.contents(),
                },
            ));

            result
        }
        (_, _) => panic!("Invalid configuration reached"),
    }
}

// TODO: In the future, these functions should be moved to a separate module, to allow customization of the matching algorithm
fn compute_matching_score_for_field_declaration<'a>(left: &'a CSTNode, right: &'a CSTNode) -> usize {
    match (left, right) {
        (
            CSTNode::Terminal(Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            CSTNode::Terminal(Terminal {
                kind: kind_right,
                value: value_right,
                ..
            }),
        ) => (kind_left == kind_right && value_left == value_right).into(),
        (
            CSTNode::NonTerminal(NonTerminal {
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                children: children_right,
                ..
            }),
        ) => {
            // Try to find an identifier on children, and compare them
            let variable_declarator_left = children_left.iter().find(|node| match node {
                CSTNode::NonTerminal(NonTerminal { kind, .. }) => kind == &"variable_declarator",
                _ => false,
            }).map(|node | {
                match node {
                    CSTNode::NonTerminal(non_terminal) => non_terminal,
                    CSTNode::Terminal(_) => panic!("Invalid configuration reached"),
                }
            });

            let variable_declarator_right = children_right.iter().find(|node| match node {
                CSTNode::NonTerminal(NonTerminal { kind, .. }) => kind == &"variable_declarator",
                _ => false,
            }).map(|node | {
                match node {
                    CSTNode::NonTerminal(non_terminal) => non_terminal,
                    CSTNode::Terminal(_) => panic!("Invalid configuration reached"),
                }
            });

            // Try to find an identifier on children, and compare them
            let identifier_left = variable_declarator_left.unwrap().children.iter().find(|node| match node {
                CSTNode::Terminal(Terminal { kind, .. }) => kind == &"identifier",
                _ => false,
            });

            let identifier_right = variable_declarator_right.unwrap().children.iter().find(|node| match node {
                CSTNode::Terminal(Terminal { kind, .. }) => kind == &"identifier",
                _ => false,
            });

            println!("identifier_left: {:?}", identifier_left);
            println!("identifier_right: {:?}", identifier_right);

            match (identifier_left, identifier_right) {
                (Some(identifier_left), Some(identifier_right)) => {
                    match (identifier_left, identifier_right) {
                        (
                            CSTNode::Terminal(Terminal {
                                value: value_left, ..
                            }),
                            CSTNode::Terminal(Terminal {
                                value: value_right, ..
                            }),
                        ) if value_left == value_right => 1,
                        (_, _) => 0,
                    }
                }
                (_, _) => 0,
            }
        }
        (_, _) => 0,
    }    
}

// TODO: In the future, these functions should be moved to a separate module, to allow customization of the matching algorithm
fn compute_matching_score<'a>(left: &'a CSTNode, right: &'a CSTNode) -> usize {
    println!("Comparing node of kind {:?} with {:?}", left.kind(), right.kind());

    match (left, right) {
        (
            CSTNode::Terminal(Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            CSTNode::Terminal(Terminal {
                kind: kind_right,
                value: value_right,
                ..
            }),
        ) => (kind_left == kind_right && value_left == value_right).into(),
        (
            CSTNode::NonTerminal(NonTerminal {
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                children: children_right,
                ..
            }),
        ) => {
            // Try to find an identifier on children, and compare them
            let identifier_left = children_left.iter().find(|node| match node {
                CSTNode::Terminal(Terminal { kind, .. }) => kind == &"identifier",
                _ => false,
            });

            let identifier_right = children_right.iter().find(|node| match node {
                CSTNode::Terminal(Terminal { kind, .. }) => kind == &"identifier",
                _ => false,
            });

            match (identifier_left, identifier_right) {
                (Some(identifier_left), Some(identifier_right)) => {
                    match (identifier_left, identifier_right) {
                        (
                            CSTNode::Terminal(Terminal {
                                value: value_left, ..
                            }),
                            CSTNode::Terminal(Terminal {
                                value: value_right, ..
                            }),
                        ) if value_left == value_right => 1,
                        (_, _) => 0,
                    }
                }
                (_, _) => 0,
            }
        }
        (_, _) => 0,
    }
}
