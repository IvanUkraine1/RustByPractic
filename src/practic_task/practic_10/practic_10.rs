fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    let n = ((n % len as isize) + len as isize) % len as isize;
    let (left, right) = s.split_at(len - n as usize);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [0, 1, 2, 8, -1, -2, -8, 10, -10];

    for &n in &shifts {
        let rotated = rotate(s.clone(), n);
        println!("Перевертаємо ({}, {}) = {}", s, n, rotated);
    }
}