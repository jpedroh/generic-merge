use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct UnorderedPair<T> {
    pub left: T,
    pub right: T,
}

impl<T: Ord> UnorderedPair<T> {
    pub fn new(a: T, b: T) -> Self {
        if a < b {
            Self { left: a, right: b }
        } else {
            Self { left: b, right: a }
        }
    }
}

impl<T: Ord> From<(T, T)> for UnorderedPair<T> {
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<T: PartialEq<T>> PartialEq<UnorderedPair<T>> for UnorderedPair<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.left == rhs.left && self.right == rhs.right
    }
}

impl<T: Eq> Eq for UnorderedPair<T> {}

impl<T: PartialOrd> PartialOrd for UnorderedPair<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.left.partial_cmp(&rhs.left) {
            Some(Ordering::Equal) => self.right.partial_cmp(&rhs.right),
            v => v,
        }
    }
}

impl<T: Ord> Ord for UnorderedPair<T> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self.left.cmp(&rhs.left) {
            Ordering::Equal => self.right.cmp(&rhs.right),
            v => v,
        }
    }
}

impl<T: Hash> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.left.hash(hasher);
        self.right.hash(hasher);
    }
}
