fn gcd(a: u32, b: u32) -> u32 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

#[test]
fn test_gcd() {
    let data = [
        ((24, 60), 12),
        ((15, 9), 3),
        ((15, 6), 3),
        ((140, 40), 20),
        ((24, 16), 8),
        ((100, 10), 10),
        ((120, 80), 40),
        ((80, 120), 40),
        ((100, 20), 20),
        ((37, 11), 1),
        ((120, 90), 30),
    ];

    for ((a, b), exp) in data.iter() {
        let result = gcd(*a, *b);
        println!("gcd({}, {}) = {}, результат {}", a, b, result, exp);
        assert_eq!(*exp, gcd(*a, *b));
    }
}

