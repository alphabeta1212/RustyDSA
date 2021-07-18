pub fn insertion_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let arr_len = arr.len();

    for i in 1..arr_len {
        let mut marker = i;
        let value = arr[i];
        while marker > 0 && arr[marker - 1] > value {
            arr[marker] = arr[marker - 1];
            marker = marker - 1;
        }
        arr[marker] = value;
    }

    arr
}
