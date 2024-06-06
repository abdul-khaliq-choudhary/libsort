use super::sort::Sorter;

pub trait SortableSlice {
    fn sort_with<S>(&mut self, sorter: S)
    where
        S: Sorter;
}

impl<T> SortableSlice for [T]
where
    T: Ord,
{
    fn sort_with<S>(&mut self, sorter: S)
    where
        S: Sorter,
    {
        sorter.sort(self);
    }
}
