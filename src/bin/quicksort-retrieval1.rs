fn create_spaces(times: usize) -> String {
    let mut s = String::with_capacity(times);
    for _ in 0..times {
        s.push(' ');
    }
    s
}

fn quick_sort(arr: &mut [i32], spaces: usize) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let my_spaces = create_spaces(spaces);
    let pivot_index = partition(arr, spaces + 1);

    let before1 = arr[0..pivot_index].iter().copied().collect::<Vec<i32>>();
    let before2 = arr[pivot_index + 1..len]
        .iter()
        .copied()
        .collect::<Vec<i32>>();
    quick_sort(&mut arr[0..pivot_index], spaces + 2);
    quick_sort(&mut arr[pivot_index + 1..len], spaces + 2);
    let after1 = arr[0..pivot_index].iter().copied().collect::<Vec<i32>>();
    let after2 = arr[pivot_index + 1..len]
        .iter()
        .copied()
        .collect::<Vec<i32>>();
    println!(
        "{}Sorted partition ({}) - 1: {:?} -> {:?}, 2: {:?} -> {:?}",
        my_spaces, pivot_index, before1, after1, before2, after2
    );
}

fn partition(arr: &mut [i32], spaces: usize) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;

    let my_spaces = create_spaces(spaces);

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            let before = arr.iter().copied().collect::<Vec<i32>>();
            let value = arr[j];
            arr.swap(i, j);
            println!(
                "{}Swapping value {} from {} to {} - pivot {}(@{}) - Before: {:?} -> {:?}",
                my_spaces,
                value,
                j,
                i,
                pivot,
                len - 1,
                before,
                arr
            );

            i += 1;
        }
    }

    arr.swap(i, len - 1);
    println!("{}Returning partition @{} -> {:?}", my_spaces, i, arr);
    i
}

fn main() {
    let mut vec = vec![3, 6, 8, 10, 1, 2, 1];
    println!("Original array: {:?}", vec);
    quick_sort(&mut vec, 1);
    println!("Sorted array: {:?}", vec);
}

mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut input = [5, 4, 3, 2, 1];
        quick_sort(&mut input, 0);
        assert_eq!(input, [1, 2, 3, 4, 5]);
        input[2] = 1;
    }
}
