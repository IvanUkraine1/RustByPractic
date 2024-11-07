//https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=true

use std::io::{self, BufRead};

fn plusMinus(arr: &[i32]) {
    let n = arr.len() as f64;

    let (mut positive_count, mut negative_count, mut zero_count) = (0, 0, 0);

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    println!("{:.6}", positive_count as f64 / n);
    println!("{:.6}", negative_count as f64 / n);
    println!("{:.6}", zero_count as f64 / n);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}

#[test]
fn test_plus_minus() {
    let arr = vec![1, -1, 0];
    plusMinus(&arr);
}
