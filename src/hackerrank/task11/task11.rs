//https://www.hackerrank.com/challenges/apple-and-orange/problem?isFullScreen=true

use std::io::{self, BufRead};

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    for &apple in apples {
        let landing_position = a + apple;
        if landing_position >= s && landing_position <= t {
            apple_count += 1;
        }
    }

    let mut orange_count = 0;
    for &orange in oranges {
        let landing_position = b + orange;
        if landing_position >= s && landing_position <= t {
            orange_count += 1;
        }
    }

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the input values
    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let s = first_multiple_input[0];
    let t = first_multiple_input[1];

    let second_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let a = second_multiple_input[0];
    let b = second_multiple_input[1];

    let third_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let _m = third_multiple_input[0]; // Number of apples (not needed)
    let _n = third_multiple_input[1]; // Number of oranges (not needed)

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}

#[test]
fn test_count_apples_and_oranges() {
    let s = 7;
    let t = 10;
    let a = 4;
    let b = 12;
    let apples = vec![2, 3, -4];
    let oranges = vec![3, -2, -4];

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}
