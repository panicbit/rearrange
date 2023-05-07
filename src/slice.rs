use crate::Order;

pub trait Rearrange<T> {
    fn rearrange<F>(self, f: F)
    where
        T: Ord,
        F: for<'x> FnMut(Order<'x, T>) -> Order<'x, T>;
}

impl<T> Rearrange<T> for &mut [T] {
    fn rearrange<F>(self, f: F)
    where
        T: Ord,
        F: for<'x> FnMut(Order<'x, T>) -> Order<'x, T>,
    {
        rearrange(self, f)
    }
}

fn rearrange<T, F>(arr: &mut [T], mut f: F)
where
    T: Ord,
    F: for<'x> FnMut(Order<'x, T>) -> Order<'x, T>,
{
    arr.sort_by(|a, b| {
        let order = Order::new(a, b);
        let order = f(order);

        order.ordering()
    });
}
