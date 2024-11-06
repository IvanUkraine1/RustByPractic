/// https://practice.course.rs/basic-types/numbers.html

#[test]
fn test411() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    let z = 10;

    println!("Success!");
}

#[test]
fn test412() {
        let v: u16 = 38_u8 as u16;

        println!("Success!");
}

#[test]
fn test413() {
        let x = 5;
        assert_eq!("i32".to_string(), type_of(&x));

        println!("Success!");

    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

}

#[test]
fn test414() {
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);

        println!("Success!");
}

#[test]
fn test415() {
        let v1 = 251_u16 + 8;
        let v2 = i16::checked_add(251, 8).unwrap();

        println!("{}, {}", v1, v2);
}

#[test]
fn test416() {
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        assert!(v == 1597);

        println!("Success!");
}

#[test]
fn test417() {
        let x = 1_000.000_1;
        let y: f32 = 0.12;
        let z = 0.01_f64;

        assert_eq!(type_of(&x), "f64".to_string());
        println!("Success!");

    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}

#[test]
fn test418() {
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < 0.0001);

    println!("Success!");
}

#[test]
fn test419() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}

#[test]
fn test4110() {
    use std::ops::{Range, RangeInclusive};

        assert_eq!((1..5), Range { start: 1, end: 5 });
        assert_eq!((1..=5), RangeInclusive::new(1, 5));

        println!("Success!");
}

#[test]
fn test4111() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(3 * 50 == 150);
    assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < 1e-10);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

#[test]
fn test421() {
    use std::mem::size_of_val;

        let c1 = 'a';
        assert_eq!(size_of_val(&c1), 4);

        let c2 = '中';
        assert_eq!(size_of_val(&c2), 4);

        println!("Success!");
}

#[test]
fn test422() {
    let c1 = '中';
    print_char(c1);

fn print_char(c: char) {
    println!("{}", c);
}
}

#[test]
fn test423() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test424() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test425() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
fn implicitly_ret_unit() {
    println!("I will return a ()");
}
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
}

#[test]
fn test426() {
    use std::mem::size_of_val;

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

#[test]
fn test431() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

#[test]
fn test432() {
    let v = 3;
    assert!(v == 3);
    println!("Success!");
}

#[test]
fn test433() {
    let s = sum(1, 2);
    assert_eq!(s, 3);
    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

#[test]
fn test441() {
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    println!("Success!");
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
}

#[test]
fn test442() {
    let message = print();
    println!("{}", message);

fn print() -> String {
    println!("Success!");
    String::from("Printed Success!")
}
}

#[test]
fn test443() {
//     never_return();
//
//     println!("Failed!");
// }
//     fn never_return() -> ! {
//     loop {
//
//     }
}

#[test]
fn test444() {
    println!("Success!");
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42),
        _ => None,
    }
}
fn never_return_fn() -> ! {
    std::process::exit(1);
}
}

#[test]
fn test445() {
        let b = false;

        let _v = match b {
            true => 1,
            false => {
                println!("Success! No panic this time.");
                0
            }
        };

        println!("This line will print if b is false!");
}