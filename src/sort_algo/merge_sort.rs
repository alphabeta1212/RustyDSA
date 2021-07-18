fn merge(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();

    while a.len() > 0 && b.len() > 0 {
        if a[0] > b[0] {
            c.push(b[0]);
            b.remove(0);
        } else {
            c.push(a[0]);
            a.remove(0);
        }
    }

    while a.len() > 0 {
        c.push(a[0]);
        a.remove(0);
    }

    while b.len() > 0 {
        c.push(b[0]);
        b.remove(0);
    }

    c
}

pub fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    if len == 1 {
        return arr;
    }

    let mut a = arr[0..len / 2].to_vec();
    let mut b = arr[(len / 2)..].to_vec();
    a = merge_sort(a);
    b = merge_sort(b);

    merge(a, b)
}
