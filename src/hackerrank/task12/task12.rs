//https://www.hackerrank.com/challenges/kangaroo/problem?isFullScreen=true

use std::io::{self, BufRead, Write};
use std::env;
use std::fs::File;

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            return String::from("YES");
        } else {
            return String::from("NO");
        }
    } else {
        if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) >= 0 {
            return String::from("YES");
        } else {
            return String::from("NO");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();
    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn test_kangaroo() {
    let x1 = 0;
    let v1 = 3;
    let x2 = 4;
    let v2 = 2;
    let result = kangaroo(x1, v1, x2, v2);
    assert_eq!(result, "YES");
}
