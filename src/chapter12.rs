#[test]
fn test1211(){
        let decimal = 97.123_f32;

        let integer: u8 = decimal as u8;

        let c1: char = integer as char;
        let c2 = integer as char;

        assert_eq!(integer, 'a' as u8);

        println!("Success!");
}

#[test]
fn test1212(){
        assert_eq!(u8::MAX, 255);
        let v = 255 as u8;

        println!("Success!");

}

#[test]
fn test1213(){
        assert_eq!(1000 as u16, 1000);

        assert_eq!(1000 % 256, 232);

        println!("1000 mod 256 is : {}", 1000 % 256);

        assert_eq!(-1_i8 as u8, 255);

        assert_eq!(300.1_f32 as u8, 255);
        assert_eq!(-100.1_f32 as u8, 0);

        unsafe {
            println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
            println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
            println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
        }
}

#[test]
fn test1214(){
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address: usize = p1 as usize;
        let second_address = first_address + 4;
        let p2: *mut i32 = second_address as *mut i32;
        unsafe {
            *p2 += 1;
        }

        assert_eq!(values[1], 3);

        println!("Success!");
}

#[test]
fn test1215(){
        let arr: [u64; 13] = [0; 13];
        assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
        let a: *const [u64; 13] = &arr;
        let b = a as *const [u8; 104];
        unsafe {
            assert_eq!(std::mem::size_of_val(&*b), 104)
        }

        println!("Success!");
}

#[test]
fn test1221(){
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        let i3: i32 = 'a' as i32;

        let s: String = 'a'.to_string();

        println!("Success!");
}

#[test]
fn test1222(){
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        let i3: i32 = 'a' as i32;

        let s: String = 'a'.to_string();

        println!("Success!");
}

#[test]
fn test1223(){
    use std::fs;
    use std::io;
    use std::num;

    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError {
            CliError::IoError(err)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError {
            CliError::ParseError(err)
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        let contents = fs::read_to_string(&file_name)?;
        let num: i32 = contents.trim().parse()?;
        Ok(num)
    }
        match open_and_parse_file("file.txt") {
            Ok(num) => println!("Parsed number: {}", num),
            Err(e) => match e {
                CliError::IoError(err) => eprintln!("I/O error: {}", err),
                CliError::ParseError(err) => eprintln!("Parse error: {}", err),
            },
        }
}

#[test]
fn test1224(){
    use std::convert::TryInto;

        let n: i16 = 256;
        let n: u8 = match n.try_into() {
            Ok(n) => n,
            Err(e) => {
                println!("there is an error when converting: {:?}, but we catch it", e.to_string());
                0
            }
        };
        assert_eq!(n, 0);

        println!("Success!");
}


#[test]
fn test1225(){
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(())
            }
        }
    }

        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(EvenNum::try_from(5), Err(()));

        let result: Result<EvenNum, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));
        let result: Result<EvenNum, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));

        println!("Success!");
}

#[test]
fn test1231(){
    use std::fmt;

    struct Point1 {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point1 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

        let origin = Point1 { x: 0, y: 0 };

        assert_eq!(origin.to_string(), "The point is (0, 0)");
        assert_eq!(format!("{}", origin), "The point is (0, 0)");

        println!("Success! 1");
}

#[test]
fn test1232(){
    use std::str::FromStr;

        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed: i32 = "10".parse().unwrap();
        let from_str = i32::from_str("20").unwrap();

        let sum = parsed + turbo_parsed + from_str;
        assert_eq!(sum, 35);

        println!("Success!");
}

#[test]
fn test1233(){
    use std::str::FromStr;
    use std::num::ParseIntError;

    #[derive(Debug, PartialEq)]
    struct Point3 {
        x: i32,
        y: i32,
    }

    impl FromStr for Point3 {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .map(|x| x.trim())
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point3 { x: x_fromstr, y: y_fromstr })
        }
    }

        let p = Point3::from_str("(3, 4)");

        assert_eq!(p.unwrap(), Point3 { x: 3, y: 4 });
        println!("Success!");
}