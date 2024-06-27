use std::{fmt::Debug, hash::Hash, marker::PhantomData};

pub struct Index<T>(pub usize, pub PhantomData<T>);
impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Index<T> {}
impl<T> Debug for Index<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index({})", self.0)
    }
}
impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Index<T> {}
impl<T> Hash for Index<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
impl<T> Index<T> {
    pub fn new(i: usize) -> Self {
        Self(i, PhantomData)
    }
}

#[derive(Clone)]
pub struct IndexVec<T>(Vec<T>, PhantomData<Index<T>>);
impl<T> Default for IndexVec<T> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<T> std::ops::Index<Index<T>> for IndexVec<T> {
    type Output = T;

    fn index(&self, index: Index<T>) -> &Self::Output {
        self.0.index(index.0)
    }
}
impl<T> IndexVec<T> {
    pub fn push(&mut self, value: T) -> Index<T> {
        let i = Index::new(self.0.len());

        self.0.push(value);

        i
    }

    pub fn get(&self, index: Index<T>) -> Option<&T> {
        self.0.get(index.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = Index<T>> {
        (0..self.0.len()).map(|i| Index::new(i))
    }
}
