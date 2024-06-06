use super::sort::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        'outer: for i in 0..slice.len() {
            let mut swapped = false;

            for j in 0..slice.len() - i - 1 {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break 'outer;
            }
        }
    }
}
