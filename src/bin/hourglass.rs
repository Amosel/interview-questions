use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'hourglass_sum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn hourglass_sum(arr: &[Vec<i32>]) -> i32 {
    // arr[i][j] = -9..9;
    // i,k = 0..5
    //
    let mut sums: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let indices_lookup: [usize; 7] = [0, 1, 2, 7, 12, 13, 14];

    // (0,0), (0,1), (0,2)
    // (1,1)
    // (2,0), (2,1), (2,2)

    // (0,1), (0,2), (0,3)
    // (1,2)
    // (2,1), (2,2), (2,3)

    // (0,2), (0,3), (0,4)
    // (1,3)
    // (2,2), (2,3), (2,4)

    // (0,3), (0,4), (0,5)
    // (1,4)
    // (2,3), (2,4), (2,5)

    // (1,0), (1,1), (1,2)
    // (2,1)
    // (3,0), (3,1), (3,2)

    // (1,1), (1,2), (1,3)
    // (2,2)
    // (3,1), (3,2), (3,3)

    // (1,2), (1,3), (1,4)
    // (2,3)
    // (3,2), (3,3), (3,4)

    // (1,3), (1,4), (1,5)
    // (2,4)
    // (3,3), (3,4), (3,5)

    for sums_index in 0..sums.len() {
        let mut sum = 0;
        for lookup_index in indices_lookup {
            let i = lookup_index / 6 + (sums_index / 4);
            let j = lookup_index % 6 + (sums_index % 4);
            let v = arr.get(i).unwrap().get(j).unwrap();
            sum += v;
        }
        sums[sums_index] = sum;
    }
    *sums.iter().max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    println!("Hi mom");
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    println!("Opening {:?}", fptr);
    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(6_usize);

    for i in 0..6_usize {
        arr.push(Vec::with_capacity(6_usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    // for (i, subarray) in arr.iter().enumerate() {
    //     for (j, value) in subarray.iter().enumerate() {
    //         let index = j + (i * 6);
    //         println!("index: {}, i: {}, j: {}, v: {}", index, i, j, value);
    //     }
    // }
    let result = hourglass_sum(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
