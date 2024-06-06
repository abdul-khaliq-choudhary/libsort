use super::{BubbleSort, SortableSlice};

#[test]
fn test_bubble_sort() {
    let mut vec = [3, 2, 5, 1, 4];
    vec.sort_with(BubbleSort);
    assert_eq!(&vec, &[1, 2, 3, 4, 5]);
}
