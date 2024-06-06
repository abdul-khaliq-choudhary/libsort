mod bubble_sort;
mod sort;
mod sortable_slice;

pub use crate::bubble_sort::BubbleSort;
pub use crate::sortable_slice::SortableSlice;
pub use sort::Sorter;

#[cfg(test)]
mod tests;
