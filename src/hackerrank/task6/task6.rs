//https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true

use std::io::{self, BufRead};

fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;

        let row = " ".repeat(spaces as usize) + &"#".repeat(hashes as usize);

        println!("{}", row);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}

#[test]
fn test_staircase() {
    let n = 3;
    staircase(n);
}
