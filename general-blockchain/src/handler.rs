#![allow(non_snake_case)]
use std::{
    io::stdin,
    str,
    error::Error,
};

pub struct Scanner {
    pub input: String,
}

impl Scanner {
    fn new(s: String) -> Scanner {
        Scanner {
            input: s.trim().to_string(),
        }
    }

    pub fn to_u64(mut self) -> Result<u64, Box<dyn Error>> {
        Ok(self.input.parse::<u64>().expect("please input correct number"))
    }
}

pub fn input() -> Scanner {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    Scanner::new(input)
}