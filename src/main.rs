mod sort_algo;
use crate::sort_algo::{bubble::bubble_sort, insert_sort::insertion_sort, merge_sort::merge_sort};
fn main() {
    let mut arr = vec![1, 7, 3, 7, 23, 8, 2, 3, 9];
    // arr = insertion_sort(arr);
    arr = merge_sort(arr);
    println!("Sorted array looks like : {:?}", arr);
}
