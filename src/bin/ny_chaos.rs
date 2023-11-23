use std::io::{self, BufRead};

fn minimum_bribes(q: Vec<i32>) {
    let mut total_bribes = 0;

    for i in 0..q.len() {
        // Check if the current person has moved more than 2 positions forward
        if q[i] - (i as i32 + 1) > 2 {
            println!("Too chaotic");
            return;
        }

        // Count the number of bribes received by the current person
        for j in 0.max(q[i] - 2) as usize..i {
            if q[j] > q[i] {
                total_bribes += 1;
            }
        }
    }

    println!("{}", total_bribes);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let _: i32 = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let q: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        minimum_bribes(q);
    }
}
