//https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true

use std::io::{self, BufRead};

fn miniMaxSum(arr: &[i64]) {
    let total_sum: i64 = arr.iter().sum();
    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();

    let min_sum = total_sum - max_value;

    let max_sum = total_sum - min_value;

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    miniMaxSum(&arr);
}

#[test]
fn test_mini_max_sum() {
    let arr = vec![1, 2, 3, 4, 5];
    miniMaxSum(&arr);
}