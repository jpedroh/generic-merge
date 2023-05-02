use model::CSTNode;
use std::{collections::HashMap, iter::Map};

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct UnorderedPair<T> {
    a: T,
    b: T,
}

impl<T: Ord> UnorderedPair<T> {
    pub fn new(a: T, b: T) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn ref_a(&self) -> &T {
        &self.a
    }

    pub fn ref_b(&self) -> &T {
        &self.b
    }
}

impl<T: Ord> From<(T, T)> for UnorderedPair<T> {
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<T: PartialEq<T>> PartialEq<UnorderedPair<T>> for UnorderedPair<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.a == rhs.a && self.b == rhs.b
    }
}

impl<T: Eq> Eq for UnorderedPair<T> {}

impl<T: PartialOrd> PartialOrd for UnorderedPair<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.a.partial_cmp(&rhs.a) {
            Some(Ordering::Equal) => self.b.partial_cmp(&rhs.b),
            v => v,
        }
    }
}

impl<T: Ord> Ord for UnorderedPair<T> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self.a.cmp(&rhs.a) {
            Ordering::Equal => self.b.cmp(&rhs.b),
            v => v,
        }
    }
}

impl<T: Hash> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.a.hash(hasher);
        self.b.hash(hasher);
    }
}

#[derive(Clone, Debug)]
struct Matching {
    left: CSTNode,
    right: CSTNode,
    score: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    TOP,
    LEFT,
    DIAG,
}

#[derive(Clone)]
struct Entry(pub Direction, pub HashMap<UnorderedPair<CSTNode>, Matching>);

impl Default for Entry {
    fn default() -> Self {
        Self(Direction::TOP, Default::default())
    }
}

fn ordered_tree_matching(
    left: &CSTNode,
    right: &CSTNode,
) -> HashMap<UnorderedPair<CSTNode>, Matching> {
    match (left, right) {
        (
            CSTNode::NonTerminal {
                kind: kind_left,
                children: children_left,
            },
            CSTNode::NonTerminal {
                kind: kind_right,
                children: children_right,
            },
        ) => {
            let root_matching: usize = (kind_left == kind_right).into();

            let m = children_left.len();
            let n = children_right.len();

            let mut matrix_m = vec![vec![0; n + 1]; m + 1];
            let mut matrix_t = vec![vec![Entry::default(); n + 1]; m + 1];

            for i in 1..m + 1 {
                for j in 1..n + 1 {
                    let left_child = children_left.get(i - 1).unwrap();
                    let right_child = children_right.get(j - 1).unwrap();

                    let w = ordered_tree_matching(left_child, right_child);
                    let matching = w
                        .get(&UnorderedPair::new(
                            left_child.to_owned(),
                            right_child.to_owned(),
                        ))
                        .unwrap();

                    if matrix_m[i][j - 1] > matrix_m[i - 1][j] {
                        if matrix_m[i][j - 1] > matrix_m[i - 1][j - 1] + matching.score {
                            matrix_m[i][j] = matrix_m[i][j - 1];
                            matrix_t[i][j] = Entry(Direction::LEFT, w);
                        } else {
                            matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                            matrix_t[i][j] = Entry(Direction::DIAG, w);
                        }
                    } else {
                        if matrix_m[i - 1][j] > matrix_m[i - 1][j - 1] + matching.score {
                            matrix_m[i][j] = matrix_m[i - 1][j];
                            matrix_t[i][j] = Entry(Direction::TOP, w);
                        } else {
                            matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                            matrix_t[i][j] = Entry(Direction::DIAG, w);
                        }
                    }
                }
            }

            let mut i = m;
            let mut j = n;
            let mut children = Vec::<&HashMap<UnorderedPair<CSTNode>, Matching>>::new();

            while i >= 1 && j >= 1 {
                match matrix_t.get(i).unwrap().get(j).unwrap().0 {
                    Direction::TOP => i = i - 1,
                    Direction::LEFT => j = j - 1,
                    Direction::DIAG => {
                        if matrix_m[i][j] > matrix_m[i - 1][j - 1] {
                            children.push(&matrix_t[i][j].1);
                        }
                        i = i - 1;
                        j = j - 1;
                    }
                }
            }

            let matching = Matching {
                left: left.to_owned(),
                right: right.to_owned(),
                score: matrix_m[m][n] + root_matching,
            };
            let mut result = HashMap::new();
            result.insert(
                UnorderedPair::new(left.to_owned(), right.to_owned()),
                matching,
            );
            children.into_iter().for_each(|child_matchings| {
                child_matchings.iter().for_each(|(key, matching)| {
                    result.insert(key.to_owned(), matching.to_owned());
                })
            });
            result
        }
        (
            CSTNode::Terminal {
                kind: kind_left,
                value: value_left,
            },
            CSTNode::Terminal {
                kind: kind_right,
                value: value_right,
            },
        ) => {
            let mut result = HashMap::new();
            result.insert(
                UnorderedPair::new(left.to_owned(), right.to_owned()),
                Matching {
                    left: left.to_owned(),
                    right: right.to_owned(),
                    score: (kind_left == kind_right && value_left == value_right).into(),
                },
            );
            result
        }
        (_, _) => {
            let mut result = HashMap::new();
            result.insert(
                UnorderedPair::new(left.to_owned(), right.to_owned()),
                Matching {
                    left: left.to_owned(),
                    right: right.to_owned(),
                    score: 0,
                },
            );
            result
        }
    }
}

fn main() {
    let base = parsing::parse_string(
        r#"
        public static interface HelloWorld {
            void sayHello(String name);
        }
    "#,
        parsing::ParserConfiguration::from_language(model::Language::Java),
    )
    .unwrap();
    let left = parsing::parse_string(
        r#"
        public static interface HelloWorld {
            void sayHello(String name);
            void sayBye(String name);
        }
    "#,
        parsing::ParserConfiguration::from_language(model::Language::Java),
    )
    .unwrap();
    let right = parsing::parse_string(
        r#"
        public static interface HelloWorld {
            void killAllHumans();
            void sayHello(String name);
        }
    "#,
        parsing::ParserConfiguration::from_language(model::Language::Java),
    )
    .unwrap();

    let result = ordered_tree_matching(&left, &base);

    println!("{:?}", &result.get(&UnorderedPair::new(left, right)));

    result
        .into_iter()
        .for_each(|(pair, matching)| println!("{:?}", matching));
}
