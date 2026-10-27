fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let pivot = partition(arr);

    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();

    let pivot_index = median_of_three(arr);

    arr.swap(pivot_index, len - 1);

    let pivot = arr[len - 1];

    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);

    i
}

fn median_of_three(arr: &[i32]) -> usize {
    let first = 0;
    let middle = arr.len() / 2;
    let last = arr.len() - 1;

    let a = arr[first];
    let b = arr[middle];
    let c = arr[last];

    if (a <= b && b <= c) || (c <= b && b <= a) {
        middle
    } else if (b <= a && a <= c) || (c <= a && a <= b) {
        first
    } else {
        last
    }
}

fn main() {
    let mut arr = [5, 3, 8, 1, 4, 7, 2, 6];

    quick_sort(&mut arr);

    println!("{:?}", arr);
}