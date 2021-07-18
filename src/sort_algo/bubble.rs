pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len() - 1;
    for _ in 0..len {
        for j in 0..len {
            if arr[j] > arr[j + 1] {
                let temp = arr[j + 1];
                arr[j + 1] = arr[j];
                arr[j] = temp;
            }
        }
    }
    arr
}
