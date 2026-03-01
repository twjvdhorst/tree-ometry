use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;

use paste::paste;
use derive_more::{Debug, Display, From};

pub trait TreeSemigroup<K> {
    fn op(key: &K, left: Option<&Self>, right: Option<&Self>) -> Self;
}

// Implementing semigroups for tuples, elementwise.
macro_rules! impl_tuple {
    ($($T:tt)*) => {
        paste! {
            impl<K, $($T,)*> TreeSemigroup<K> for ($($T,)*)
            where
                $($T: TreeSemigroup<K>,)*
            {
                fn op(key: &K, left: Option<&Self>, right: Option<&Self>) -> Self {
                    match (left, right) {
                        (Some(($([<$T:lower _left>],)*)), Some(($([<$T:lower _right>],)*))) => (
                            $($T::op(key, Some([<$T:lower _left>]), Some([<$T:lower _right>])),)*
                        ),
                        (Some(($([<$T:lower _left>],)*)), None) => (
                            $($T::op(key, Some([<$T:lower _left>]), None),)*
                        ),
                        (None, Some(($([<$T:lower _right>],)*))) => (
                            $($T::op(key, None, Some([<$T:lower _right>])),)*
                        ),
                        (None, None) => (
                            $($T::op(key, None, None),)*
                        ),
                    }
                }
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A B);
impl_tuple!(A B C);
impl_tuple!(A B C D);
impl_tuple!(A B C D E);
impl_tuple!(A B C D E F);
impl_tuple!(A B C D E F G);
impl_tuple!(A B C D E F G H);
impl_tuple!(A B C D E F G H I);
impl_tuple!(A B C D E F G H I J);

/// Semigroup encoding the size (number of nodes) of a subtree.
#[derive(Clone, Copy, Debug, Display, From, PartialEq, Eq, PartialOrd, Ord)]
#[debug("{_0:?}")]
#[display("{_0}")]
pub struct Size(usize);
impl<K> TreeSemigroup<K> for Size {
    fn op(_key: &K, left: Option<&Self>, right: Option<&Self>) -> Self {
        let size_left = left.unwrap_or(&Self(0));
        let size_right = right.unwrap_or(&Self(0));
        Self(1 + size_left.0 + size_right.0)
    }
}

/// Semigroup encoding the height of a subtree.
#[derive(Clone, Copy, Debug, Display, From, PartialEq, Eq, PartialOrd, Ord)]
#[debug("{_0:?}")]
#[display("{_0}")]
pub struct Height(usize);
impl<K> TreeSemigroup<K> for Height {
    fn op(_key: &K, left: Option<&Self>, right: Option<&Self>) -> Self {
        let height_left = left.unwrap_or(&Self(0));
        let height_right = right.unwrap_or(&Self(0));
        Self(1 + usize::max(height_left.0, height_right.0))
    }
}

/// Semigroup encoding the canonical interval (min and max key) of a subtree.
#[derive(Clone, Debug, Display, From, PartialEq, Eq, PartialOrd, Ord)]
#[debug("[{_0:?}, {_1:?}]")]
#[display("[{_0}, {_1}]")]
pub struct CanonInterval<K>(K, K);
impl<K> TreeSemigroup<K> for CanonInterval<K>
where 
    K: Clone + Ord,
{
    fn op(key: &K, left: Option<&Self>, right: Option<&Self>) -> Self {
        match (left, right) {
            (Some(i1), Some(i2)) => Self(i1.0.clone(), i2.1.clone()),
            (Some(i1), None) => Self(i1.0.clone(), key.clone()),
            (None, Some(i2)) => Self(key.clone(), i2.1.clone()),
            (None, None) => Self(key.clone(), key.clone()),
        }
    }
}

/// Semigroup encoding the canonical subset (all keys) of a subtree.
#[derive(Clone, From)]
pub struct CanonSubset<K>(HashSet<K>);
impl<K> TreeSemigroup<K> for CanonSubset<K>
where 
    K: Clone + Eq + Hash,
{
    fn op(key: &K, left: Option<&Self>, right: Option<&Self>) -> Self {
        let subset_left = left.map(|subset| subset.0.clone()).unwrap_or(HashSet::new());
        let subset_right = right.map(|subset| subset.0.clone()).unwrap_or(HashSet::new());
        let mut union = HashSet::union(&subset_left, &subset_right)
            .map(Clone::clone)
            .collect::<HashSet<_>>();
        union.insert(key.clone());
        Self(union)
    }
}

impl<K> PartialEq for CanonSubset<K>
where 
    K: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K> Eq for CanonSubset<K>
where 
    K: Eq + Hash,
{}

impl<K> fmt::Debug for CanonSubset<K>
where 
    K: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, key) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{key:?}")?;
        }
        write!(f, "}}")
    }
}

impl<K> fmt::Display for CanonSubset<K>
where 
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, key) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{key}")?;
        }
        write!(f, "}}")
    }
}
