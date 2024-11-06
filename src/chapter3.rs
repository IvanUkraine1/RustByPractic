/// https://practice.course.rs/variables.html
#[test]
fn test31() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test32() {
        let mut x: i32 = 1;
        x += 2;

        assert_eq!(x, 3);
        println!("Success!");
}

#[test]
fn test33() {
        let x: i32 = 10;
        {
            let y: i32 = 5;
            println!("The value of x is {} and value of y is {}", x, y);
        }
        println!("The value of x is {}", x);
}

#[test]
fn test34() {
    const X: &str = "hello";
    println!("{}, world", X);
}

#[test]
fn test35() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x);
}

#[test]
fn test36() {
        let mut x: i32 = 1;
        x = 7;
        let x = x;

        let y = 4;
        let y = "I can also be bound to text!";

        println!("Success!");
}

#[test]
fn test37() {
        let x = 1;
        println!("The value of x is {}", x);
}

#[test]
fn test38() {
        let (mut x, y) = (1, 2);
        x += 2;
        assert_eq!(x, 3);
        assert_eq!(y, 2);
        println!("Success!");
}

#[test]
fn test39() {
        let (x, y);
        (x, _) = (3, 4);
        let array = [1, 2];
        (_, y) = (array[0], array[1]);

        assert_eq!([x, y], [3, 2]);

        println!("Success!");
}