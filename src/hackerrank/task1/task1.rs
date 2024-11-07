//https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true

use std::io;

fn simple_array_sum(ar: Vec<i32>) -> i32 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let ar: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    let result = simple_array_sum(ar);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_array_sum() {
        let ar = vec![1, 2, 3, 4, 10, 11];
        let expected_sum = 31;
        assert_eq!(simple_array_sum(ar), expected_sum);
    }
}

