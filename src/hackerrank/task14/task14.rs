//https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut max_score = scores[0];
    let mut min_score = scores[0];

    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        } else if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[test]
fn test_breaking_records() {
    let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
    let result = breakingRecords(&scores);
    assert_eq!(result, vec![2, 4]);
}
