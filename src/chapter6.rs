#[test]
fn test611() {
        let s: &str = "hello, world";

        println!("Success!");
}

#[test]
fn test612() {
        let s: Box<str> = "hello, world".into();
        greetings(s);

    fn greetings(s: Box<str>) {
        println!("{}", s);
    }
}

#[test]
fn test613() {
        let mut s = String::new();
        s.push_str("hello, world");
        s.push('!');

        assert_eq!(s, "hello, world!");

        println!("Success!");
}

#[test]
fn test614(){
        let mut s = String::from("hello");
        s.push(',');
        s.push_str(" world");
        s += "!";

        println!("{}", s)
}

#[test]
fn test615(){
        let s = String::from("I like dogs");
        let s1 = s.replace("dogs", "cats");

        assert_eq!(s1, "I like cats");
        println!("Success!");
}

#[test]
fn test616(){
        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2;
        assert_eq!(s3,"hello,world!");
        println!("{}",s1);
}

#[test]
fn test617(){
        let s = "hello, world".to_string();
        greetings(s);
    fn greetings(s: String) {
        println!("{}",s)
    }
}

#[test]
fn test618(){
        let s = "hello, world".to_string();
        let s1: &str = &s;
        println!("Success!");
}

#[test]
fn test619(){
        let byte_escape = "I'm writing Ru\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!("Unicode character {} (U+211D) is called {}",
                 unicode_codepoint, character_name );

        let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
        println!("{}", long_string);
}

#[test]
fn test6110(){
        let raw_str = "Escapes don't work here: \x3F \u{211D}";
        assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        let  delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", delimiter);

        let long_delimiter = r###"Hello, "##""###;
        assert_eq!(long_delimiter, "Hello, \"##\"");
        println!("Success!");
}

#[test]
fn test6111(){
        let s1 = String::from("hi,中国");
        let h = &s1[0..1];
        assert_eq!(h, "h");

        let h1 = &s1[3..6];
        assert_eq!(h1, "中");
        println!("Success!");
}

#[test]
fn test6112(){
        for c in "你好，世界".chars() {
            println!("{}", c)
        }
}


#[test]
fn test621(){
        let arr: [i32; 5] = [1, 2, 3, 4, 5];

        assert!(arr.len() == 5);
        println!("Success!");
}

#[test]
fn test622(){
        let arr0 = [1, 2, 3];
        let arr: [_; 3] = ['a', 'b', 'c'];

        assert!(std::mem::size_of_val(&arr) == 12);
        println!("Success!");
}

#[test]
fn test623(){
        let list: [i32; 100] = [1; 100];

        assert!(list[0] == 1);
        assert!(list.len() == 100);
        println!("Success!");
}

#[test]
fn test624(){
        let _arr = [1, 2, 3];
        println!("Success!");
}

#[test]
fn test625(){
        let arr = ['a', 'b', 'c'];

        let ele = arr[0];

        assert!(ele == 'a');
        println!("Success!");
}

#[test]
fn test626(){
        let names = [String::from("Sunfei"), "Sunface".to_string()];

        let name0 = names.get(0).unwrap();

        let _name1 = &names[1];
        println!("Success!");
}

#[test]
fn test631(){
        let arr = [1, 2, 3];
        let s1: &[i32] = &arr[0..2];

        let s2: &str = "hello, world";
        println!("Success!");
}

#[test]
fn test632(){
        let arr: [char; 3] = ['中', '国', '人'];

        let slice = &arr[..2];

        assert!(std::mem::size_of_val(&slice) == 16);
        println!("Success!");
}

#[test]
fn test633(){
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let slice: &[i32] = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
        println!("Success!");
}

#[test]
fn test634(){
        let s = String::from("hello");

        let slice1 = &s[0..2];
        let slice2 = &s[..2];

        assert_eq!(slice1, slice2);
        println!("Success!");
}

#[test]
fn test635(){
        let s = "你好，世界";
        let slice = &s[0..3];

        assert!(slice == "你");
        println!("Success!");
}

#[test]
fn test636(){
        let mut s = String::from("hello world");

        let word = first_word(&s);

        println!("the first word is: {}", word);

        s.clear();
    fn first_word(s: &str) -> &str {
        &s[..1]
    }
}


#[test]
fn test641(){
        let _t0: (u8,i16) = (0, -1);
        let _t1: (u8, (i16, u32)) = (0, (-1, 1));
        let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
        println!("Success!");
}

#[test]
fn test642(){
        let t = ("i", "am", "sunface");
        assert_eq!(t.2, "sunface");
        println!("Success!");
}

#[test]
fn test643(){
        let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
        println!("too long tuple: {:?}", too_long_tuple);
}

#[test]
fn test644(){
        let tup = (1, 6.4, "hello");

        let (x, z, y) = tup;

        assert_eq!(x, 1);
        assert_eq!(y, "hello");
        assert_eq!(z, 6.4);
        println!("Success!");
}

#[test]
fn test645(){
        let (x, y, z);

        (y, z, x) = (1, 2, 3);

        assert_eq!(x, 3);
        assert_eq!(y, 1);
        assert_eq!(z, 2);
        println!("Success!");
}

#[test]
fn test646(){
        let (x, y) = sum_multiply((2, 3));

        assert_eq!(x, 5);
        assert_eq!(y, 6);
        println!("Success!");
    fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
        (nums.0 + nums.1, nums.0 * nums.1)
    }
}

#[test]
fn test651(){

    struct Person {
        name: String,
        age: u8,
        hobby: String
    }

        let age = 30;
        let p = Person {
            name: String::from("sunface"),
            age,
            hobby: "coding".to_string()
        };
        println!("Success!");
}

#[test]
fn test652(){
    struct Unit;
    trait SomeTrait {
        fn some_behavior(&self);
    }

    impl SomeTrait for Unit {
        fn some_behavior(&self) {
            println!("Unit is doing something!");
        }
    }

    fn do_something_with_unit(u: &dyn SomeTrait) {
        u.some_behavior();
    }
        let u = Unit;
        do_something_with_unit(&u);

        println!("Success!");
}

#[test]
fn test653(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn check_color(p: Color) {
        let x = p.0;
        let y = p.1;
        let z = p.2;

        assert_eq!(x, 0);
        assert_eq!(y, 127);
        assert_eq!(z, 255);
    }
        let v = Point(0, 127, 255);
        check_color(Color(v.0, v.1, v.2));

        println!("Success!");
}

#[test]
fn test654(){
    struct Person {
        name: String,
        age: u8,
    }
        let age = 18;
        let mut p = Person {
            name: String::from("sunface"),
            age,
        };

        p.age = 30;

        p.name = String::from("sunfei");
        println!("Success!");
}

#[test]
fn test655(){
    struct Person {
        name: String,
        age: u8,
    }
        println!("Success!");
}

#[test]
fn test656(){
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            active: u.active,
            username: u.username,
            sign_in_count: u.sign_in_count,
        }
    }
        let u1 = User {
            email: String::from("someone@example.com"),
            username: String::from("sunface"),
            active: true,
            sign_in_count: 1,
        };

        let u2 = set_email(u1);

        println!("Success!");
}

#[test]
fn test657(){
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
        println!("rect1: {:?}", rect1);
}

#[test]
fn test658(){
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }
        let f = File {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string(),
        };

        let _name = &f.name;
        println!("{}, {}, {:?}", _name, f.data, f);
}


#[test]
fn test661(){
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One,
        Two,
    }

    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }
        assert_eq!(Number1::One as u8, Number::One as u8);
        assert_eq!(Number1::One as u8, Number2::One as u8);

        println!("Success! 1");
}

#[test]
fn test662(){
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

        let msg1 = Message::Move { x: 1, y: 2 };
        let msg2 = Message::Write(String::from("hello, world!"));

        println!("Success!");
}

#[test]
fn test663(){
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
        let msg = Message::Move { x: 1, y: 2 };

        if let Message::Move { x, y } = msg {
            assert_eq!(x, 1);
            assert_eq!(y, 2);
        } else {
            panic!("NEVER LET THIS RUN!");
        }

        println!("Success!");
}

#[test]
fn test664(){
    use std::fmt;

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl fmt::Display for Message {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Message::Quit => write!(f, "Quit"),
                Message::Move { x, y } => write!(f, "Move to x: {}, y: {}", x, y),
                Message::Write(text) => write!(f, "Write: {}", text),
                Message::ChangeColor(r, g, b) => write!(f, "Change Color to R: {}, G: {}, B: {}", r, g, b),
            }
        }
    }

    fn show_message(msg: &Message) {
        println!("{}", msg);
    }
        let msgs: [Message; 3] = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
        ];

        for msg in msgs.iter() {
            show_message(msg)
        }
}

#[test]
fn test665(){
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        if let Some(n) = six {
            println!("{}", n);
            println!("Success! 5");
        } else {
            println!("six is None, preventing panic!");
        }
}

#[test]
fn test666(){
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            List::Nil
        }

        fn prepend(self, elem: u32) -> List {
            List::Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                },
                List::Nil => {
                    format!("Nil")
                },
            }
        }
    }
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
}


