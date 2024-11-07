//https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true

use std::io::{self, Write};

fn time_conversion(s: &str) -> String {
    let get_hours: i32 = s[0..2].parse().unwrap();
    let time_string = &s[8..];

    if time_string == "PM" {
        if get_hours == 12 {
            return format!("{}{}", get_hours, &s[2..8]);
        } else {
            return format!("{}", get_hours + 12) + &s[2..8];
        }
    } else if time_string == "AM" && get_hours == 12 {
        return format!("00{}", &s[2..8]);
    } else {
        return s[0..8].to_string();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let result = time_conversion(&input.trim());

    io::stdout().write_all(result.as_bytes()).expect("Failed to write to stdout");
}
