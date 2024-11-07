//https://www.hackerrank.com/challenges/sock-merchant/problem?isFullScreen=true

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::env;
use std::fs::File;
use std::io::Write;

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut sock_count = HashMap::new();

    for &sock in ar.iter() {
        *sock_count.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for &count in sock_count.values() {
        pairs += count / 2;
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let ar = vec![1, 1, 2, 2, 3, 3];
        let n = ar.len() as i32;

        let result = sockMerchant(n, &ar);
        assert_eq!(result, 3);
    }
}
