/// https://practice.course.rs/ownership/ownership.html
#[test]
fn test511() {
        let x = String::from("Hello world");
        let y = x.clone();
        println!("{}, {}", x, y);
}

#[test]
fn test512() {
        let s1 = String::from("Hello world");
        let s2 = take_ownership(s1);

        println!("{}", s2);

    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
}

#[test]
fn test513() {
        let s = give_ownership();
        println!("{}", s);
    fn give_ownership() -> String {
        let s = String::from("Hello world");
        let _s = s.clone().into_bytes();
        s
    }
}

#[test]
fn test514() {
        let s = String::from("Hello World");
        print_str(&s);
        println!("{}", s);

    fn print_str(s: &String) {
        println!("{}", s);
    }
}

#[test]
fn test515() {
        let x = (1, 2, (), "hello");
        let y = x;
        println!("{:?}, {:?}", x, y);
}

#[test]
fn test516() {
    let s = String::from("Hello ");
    let mut s1 = s;
    s1.push_str("World!");
    println!("Success!");
}

#[test]
fn test517() {
        let x = Box::new(5);
        let mut y = Box::new(5);
        *y = 4;
        assert_eq!(*x, 5);
        println!("Success!");
}

#[test]
fn test518() {
        let t = (String::from("hello"), String::from("world"));
        let _s = t.0.clone();
        println!("{:?}", t);
}

#[test]
fn test519() {
        let t = (String::from("hello"), String::from("world"));
        let (s1, s2) = t.clone();
        println!("{:?}, {:?}, {:?}", s1, s2, t);
}

#[test]
fn test521() {
        let x = 5;
        let p = &x;

        println!("the memory address of x is {:p}", p);
}

#[test]
fn test522() {
        let x = 5;
        let y = &x;

        assert_eq!(5, *y);

        println!("Success!");
}

#[test]
fn test523() {
    let mut s = String::from("hello, ");
    borrow_object(&s);
    println!("Success!");
    fn borrow_object(s: &String) {}
}

#[test]
fn test524() {
        let mut s = String::from("hello, ");

        push_str(&mut s);

        println!("Success!");
    fn push_str(s: &mut String) {
        s.push_str("world");
    }
}

#[test]
fn test525() {
        let mut s = String::from("hello, ");

        let p = &mut s;

        p.push_str("world");

        println!("Success!");
}

#[test]
fn test526() {
        let c = '中';
        let r1 = &c;
        let r2 = &c;
        assert_eq!(*r1, *r2);
        assert_eq!(get_addr(r1), get_addr(r2));
        println!("Success!");
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}

#[test]
fn test527() {
        let mut s = String::from("hello");

        let r1 = &mut s;
        println!("{}", r1);
}

#[test]
fn test528() {
        let mut s = String::from("hello, ");
        borrow_object(&mut s);
        println!("Success!");
    fn borrow_object(s: &mut String) {}
}

#[test]
fn test529() {
     let mut s = String::from("hello, ");
        borrow_object(&s);
        s.push_str("world");
        println!("Success!");

    fn borrow_object(s: &String) {}
}

#[test]
fn test5210() {
        let mut s = String::from("hello, ");

        let r1 = &mut s;
        r1.push_str("world");
        println!("{}", r1);
}

#[test]
fn test5211() {
        let mut s = String::from("hello, ");
        let r1 = &mut s;
        r1.push_str("world");
        println!("{}", r1);
}