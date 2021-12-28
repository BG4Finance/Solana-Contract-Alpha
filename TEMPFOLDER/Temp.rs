/// std::{cmp::min, cmp::max}; OR use std::cmp::{min, max};
use std::cmp::*;

struct Number {
    odd: bool,
    digit: i32
}

impl Number {
    fn is_strctly_positive(self) -> bool {
        self.digit > 0
    }
}

fn main(){
    let one = Number {odd: true, digit: 3};
    let two = Number {odd: false, digit: 6};
    print_number(one);
    print_number(two);
}

fn print_number(n: Number) {
    match n {
        Number {digit:3,..} => println!("Odd number: 3"),
        Number {digit:6,..} => println!("Even number: 6"),
        Number {digit,..} => println!("dunno {}", digit),
    }
        
        /// positive?
        let minus_two = Number {
            odd:false,
            digit: -2
        };
        println!("Positive? {}", minus_two.is_strctly_positive());
}