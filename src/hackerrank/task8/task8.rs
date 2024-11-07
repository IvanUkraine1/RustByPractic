//https://www.hackerrank.com/challenges/birthday-cake-candles/problem?isFullScreen=true

use std::io::{self, BufRead};
use std::fs::File;
use std::env;
use std::io::Write;

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let max_height = *candles.iter().max().unwrap();

    let count = candles.iter().filter(|&&candle| candle == max_height).count();

    count as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn test_birthday_cake_candles() {
    let candles = vec![3, 2, 1, 3];
    let result = birthdayCakeCandles(&candles);
    assert_eq!(result, 2);
}
