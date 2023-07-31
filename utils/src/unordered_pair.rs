use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct UnorderedPair<T>(pub T, pub T);

impl<T: Ord> UnorderedPair<T> {
    pub fn new(a: T, b: T) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
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
        (self.0 == rhs.0 && self.1 == rhs.1) || (self.0 == rhs.1 && self.1 == rhs.0)
    }
}

impl<T: Eq> Eq for UnorderedPair<T> {}

impl<T: PartialOrd> PartialOrd for UnorderedPair<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.0.partial_cmp(&rhs.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&rhs.1),
            v => v,
        }
    }
}

impl<T: Ord> Ord for UnorderedPair<T> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self.0.cmp(&rhs.0) {
            Ordering::Equal => self.1.cmp(&rhs.1),
            v => v,
        }
    }
}

impl<T: Hash> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher);
        self.1.hash(hasher);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_both_have_same_order_they_are_equal() {
        let left = "This is a value";
        let right = "This is another value";
        assert_eq!(UnorderedPair(left, right), UnorderedPair(left, right))
    }

    #[test]
    fn does_not_take_order_into_account_for_equality() {
        let left = "This is a value";
        let right = "This is another value";
        assert_eq!(UnorderedPair(left, right), UnorderedPair(right, left))
    }
}
