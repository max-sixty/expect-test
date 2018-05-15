#![allow(unused_imports)]
use std::env;
use std::fmt;
use std::io;
use std::io::Write;
use std::str;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
#[macro_use]
extern crate pretty_assertions;

pub struct Expect {
    pub file_name: String,
    buf: String,
}

#[derive(Debug)]
pub struct ExpectErr {
    result: String,
    expectation: String,
}

impl Error for ExpectErr {
    fn description(&self) -> &str {
        "Something bad happened"
    }
}

impl fmt::Display for ExpectErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl Expect {
    pub fn new(file_name: String) -> Expect {
        let buf = String::new();
        Expect { file_name, buf }
    }

    pub fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn expectation(&self) -> String {
        let path = env::current_dir()
            .unwrap()
            .join("expect")
            .join(&self.file_name);
        let contents = fs::read_to_string(path).unwrap();
        contents
    }

    pub fn finish(&self) -> std::result::Result<(), ExpectErr> {
        let expectation = self.expectation();
        let result = self.buf.clone();
        if result == expectation {
            Ok(())
        } else {
            // assert_eq!(result, expectation);
            Err(ExpectErr {
                result,
                expectation,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_expect() {
        let _s = Expect::new("test1".to_string());
        let _s = Expect::new(String::from("test1"));
    }
    #[test]
    fn write_to_expect_incorrect() {
        let mut s = Expect::new(String::from("test1"));
        s.push("x");
        assert!(s.finish().is_err());
    }
    #[test]
    fn write_to_expect_multiple_correct() {
        let mut s = Expect::new(String::from("test1"));
        s.push("he");
        s.push("llo");
        assert!(s.finish().is_ok());
    }
    #[test]
    fn write_to_expect_correct() {
        let mut s = Expect::new(String::from("test1"));
        s.push("hello");
        assert!(s.finish().is_ok());
    }
}
