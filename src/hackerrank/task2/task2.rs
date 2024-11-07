//https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true

use std::io::{self, BufRead};

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    vec![alice_score, bob_score]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = compare_triplets(&a, &b);
    println!("{} {}", result[0], result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_triplets() {
        let a = vec![5, 6, 7];
        let b = vec![3, 6, 10];
        let expected = vec![1, 1];
        assert_eq!(compare_triplets(&a, &b), expected);
    }

    #[test]
    fn test_compare_triplets_all_alice() {
        let a = vec![10, 20, 30];
        let b = vec![1, 2, 3];
        let expected = vec![3, 0];
        assert_eq!(compare_triplets(&a, &b), expected);
    }

    #[test]
    fn test_compare_triplets_all_bob() {
        let a = vec![1, 2, 3];
        let b = vec![10, 20, 30];
        let expected = vec![0, 3];
        assert_eq!(compare_triplets(&a, &b), expected);
    }

    #[test]
    fn test_compare_triplets_tie() {
        let a = vec![4, 5, 6];
        let b = vec![4, 5, 6];
        let expected = vec![0, 0];
        assert_eq!(compare_triplets(&a, &b), expected);
    }
}
