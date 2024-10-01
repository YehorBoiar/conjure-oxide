use serde::{Deserialize, Serialize};
use std::ops::Deref;
// use std::iter::Ste

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConstantInt(i32);

impl ConstantInt {
    pub fn new(value: i32) -> Self {
        ConstantInt(value)
    }
}

// Implementing Deref to allow access to i32's inherent methods
impl Deref for ConstantInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Range<A>
where
    A: Ord,
{
    Single(A),
    Bounded(A, A),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Domain {
    BoolDomain,
    IntDomain(Vec<Range<ConstantInt>>),
}

impl Domain {
    /// Return a list of all possible i32 values in the domain if it is an IntDomain.
    pub fn values_i32(&self) -> Option<Vec<ConstantInt>> {
        match self {
            Domain::IntDomain(ranges) => Some(
                ranges
                    .iter()
                    .flat_map(|r| match r {
                        Range::Single(i) => vec![*i],
                        Range::Bounded(i, j) => (*i..=*j).collect(),
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    /// Return an unoptimised domain that is the result of applying a binary i32 operation to two domains.
    ///
    /// The given operator may return None if the operation is not defined for its arguments.
    /// Undefined values will not be included in the resulting domain.
    ///
    /// Returns None if the domains are not valid for i32 operations.
    pub fn apply_i32(
        &self,
        op: fn(ConstantInt, ConstantInt) -> Option<ConstantInt>,
        other: &Domain,
    ) -> Option<Domain> {
        if let (Some(vs1), Some(vs2)) = (self.values_i32(), other.values_i32()) {
            // TODO: (flm8) Optimise to use smarter, less brute-force methods
            let mut new_ranges = vec![];
            for (v1, v2) in itertools::iproduct!(vs1, vs2) {
                op(v1, v2).map(|v| new_ranges.push(Range::Single(v)));
            }
            return Some(Domain::IntDomain(new_ranges));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_product() {
        let d1 = Domain::IntDomain(vec![Range::Bounded(
            ConstantInt::new(-2),
            ConstantInt::new(1),
        )]);
        let d2 = Domain::IntDomain(vec![Range::Bounded(
            ConstantInt::new(-2),
            ConstantInt::new(1),
        )]);
        let res = d1.apply_i32(|a, b| Some(a * b), &d2).unwrap();

        if let Domain::IntDomain(ranges) = res {
            // Updated to use ConstantInt instances instead of raw integers.
            assert!(!ranges.contains(&Range::Single(ConstantInt::new(-4))));
            assert!(!ranges.contains(&Range::Single(ConstantInt::new(-3))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(-2))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(-1))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(0))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(1))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(2))));
            assert!(!ranges.contains(&Range::Single(ConstantInt::new(3))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(4))));
        } else {
            panic!("Expected IntDomain");
        }
    }

    #[test]
    fn test_negative_div() {
        let d1 = Domain::IntDomain(vec![Range::Bounded(
            ConstantInt::new(-2),
            ConstantInt::new(1),
        )]);
        let d2 = Domain::IntDomain(vec![Range::Bounded(
            ConstantInt::new(-2),
            ConstantInt::new(1),
        )]);
        let res = d1
            .apply_i32(
                |a, b| {
                    if b != ConstantInt::new(0) {
                        Some(a / b)
                    } else {
                        None
                    }
                },
                &d2,
            )
            .unwrap();

        if let Domain::IntDomain(ranges) = res {
            // Updated to use ConstantInt instances instead of raw integers.
            assert!(!ranges.contains(&Range::Single(ConstantInt::new(-4))));
            assert!(!ranges.contains(&Range::Single(ConstantInt::new(-3))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(-2))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(-1))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(0))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(1))));
            assert!(ranges.contains(&Range::Single(ConstantInt::new(2))));
            assert!(!ranges.contains(&Range::Single(ConstantInt::new(3))));
            assert!(!ranges.contains(&Range::Single(ConstantInt::new(4))));
        } else {
            panic!("Expected IntDomain");
        }
    }
}
