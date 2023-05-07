use std::vec;

use crate::{Order, Rearrange};

pub trait RearrangeIter<T> {
    fn rearranged<F>(self, f: F) -> Iter<T>
    where
        T: Ord,
        F: for<'x> FnMut(Order<'x, T>) -> Order<'x, T>;
}

impl<I, T> RearrangeIter<T> for I
where
    I: Iterator<Item = T>,
{
    fn rearranged<F>(self, f: F) -> Iter<T>
    where
        T: Ord,
        F: for<'x> FnMut(Order<'x, T>) -> Order<'x, T>,
    {
        let mut items = self.collect::<Vec<_>>();
        items.rearrange(f);

        Iter(items.into_iter())
    }
}

pub struct Iter<T>(vec::IntoIter<T>);

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
