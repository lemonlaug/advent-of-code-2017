use std::fs::File;
use std::io::Read;

fn captcha(captcha_value: &str) -> u32 {
    let mut total=0;
    const RADIX: u32 = 10;

    let len = captcha_value.len();
    
    let mut iter = captcha_value.chars().peekable();
    let first = iter.peek().unwrap().clone();
    
    loop {
        match iter.next() {
            Some(digit) => {
                match iter.peek() {
                    Some(next) => {
                        println!("{} <-> {}", digit, next);
                        if digit == *next {
                            total = total + digit.to_digit(RADIX).unwrap();
                        }
                    },
                    None => {
                        println!("digit: {}, first: {}", digit, first);
                        if digit == first
                        {
                            println!("{} <*> {}", digit, first);
                            total = total + digit.to_digit(RADIX).unwrap();
                        }
                    }
                }
            },
            None => break
        }
    }
    total
}

fn main() {

    use std::io::{self, Read};

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);

    let buffer = buffer.trim();
    
    println!("{}", buffer);
/*    let contents = String::from("1111");*/
    println!("{:?}", captcha(buffer.trim()));
}
