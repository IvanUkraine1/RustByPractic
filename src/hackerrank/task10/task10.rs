//https://www.hackerrank.com/challenges/grading/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            let next_multiple_of_five = (grade + 4) / 5 * 5;
            if grade < 38 || next_multiple_of_five - grade >= 3 {
                grade
            } else {
                next_multiple_of_five
            }
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    for (i, &grade) in result.iter().enumerate() {
        write!(&mut fptr, "{}", grade).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[test]
fn test_grading_students() {
    let grades = vec![73, 67, 38, 33];
    let result = grading_students(&grades);
    assert_eq!(result, vec![75, 67, 40, 33]);
}
