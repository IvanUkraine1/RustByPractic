//https://www.hackerrank.com/challenges/drawing-book/problem?isFullScreen=true

use std::io::{self, BufRead};
use std::env;
use std::fs::File;
use std::io::Write;

fn pageCount(n: i32, p: i32) -> i32 {
    let from_front = p / 2;

    let from_back = (n / 2) - (p / 2);

    std::cmp::min(from_front, from_back)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count() {
        let n = 5;
        let p = 4;

        let result = pageCount(n, p);
        assert_eq!(result, 0);
    }
}
