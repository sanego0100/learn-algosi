fn bubble(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut arr = vec![14, 12, 23, 5, 61, 1, 2, 45, 65];
    bubble(&mut arr);
    println!("{:?}", arr);
}