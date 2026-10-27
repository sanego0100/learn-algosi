fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = Vec::with_capacity(len);

    let (left, right) = arr.split_at(mid);

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }

    arr.copy_from_slice(&merged);
}

fn main() {
    let mut arr = [5, 3, 8, 1, 4, 7];

    merge_sort(&mut arr);

    println!("{:?}", arr);
}