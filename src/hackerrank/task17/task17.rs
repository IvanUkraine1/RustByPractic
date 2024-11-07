//https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut frequency_map = std::collections::HashMap::new();

    for &bird in arr {
        *frequency_map.entry(bird).or_insert(0) += 1;
    }

    let mut max_freq = 0;
    let mut min_bird_type = i32::MAX;

    for (&bird, &freq) in frequency_map.iter() {
        if freq > max_freq || (freq == max_freq && bird < min_bird_type) {
            max_freq = freq;
            min_bird_type = bird;
        }
    }

    min_bird_type
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn test_migratory_birds() {
    let arr = vec![1, 1, 2, 2, 3];
    let result = migratoryBirds(&arr);
    assert_eq!(result, 1);
}
