use std::cmp::Ordering;

mod iter;
mod slice;

pub use iter::RearrangeIter;
pub use slice::Rearrange;

pub struct Order<'x, T> {
    a: &'x T,
    b: &'x T,
    ordering: Ordering,
}

impl<'x, T> Order<'x, T> {
    pub fn new(a: &'x T, b: &'x T) -> Self {
        Self {
            a,
            b,
            ordering: Ordering::Equal,
        }
    }

    pub fn by<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.ordering = self.ordering.then_with(|| f(self.a, self.b));
        self
    }

    pub fn reverse_by<F>(self, mut f: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.by(|a, b| f(a, b).reverse())
    }

    pub fn by_key<F, U>(self, mut f: F) -> Self
    where
        F: FnMut(&T) -> U,
        U: Ord,
    {
        self.by(|a, b| f(a).cmp(&f(b)))
    }

    pub fn reverse_by_key<F, U>(self, mut f: F) -> Self
    where
        F: FnMut(&T) -> U,
        U: Ord,
    {
        self.reverse_by(|a, b| f(a).cmp(&f(b)))
    }

    pub fn by_ref<F, U>(self, mut f: F) -> Self
    where
        F: FnMut(&T) -> &U,
        U: Ord,
    {
        self.by(|a, b| f(a).cmp(f(b)))
    }

    pub fn reverse_by_ref<F, U>(self, mut f: F) -> Self
    where
        F: FnMut(&T) -> &U,
        U: Ord,
    {
        self.reverse_by(|a, b| f(a).cmp(f(b)))
    }

    pub fn ordering(&self) -> Ordering {
        self.ordering
    }
}
