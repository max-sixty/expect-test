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
extern crate difference;
use difference::{Changeset, Difference};

pub struct Expect {
    pub file_name: String,
    buf: String,
}

// #[derive(Debug)]
pub struct ExpectErr {
    #[allow(dead_code)]
    changeset: Changeset,
}

pub enum Mode {
    Test,
    Reset,
}

impl Error for ExpectErr {
    fn description(&self) -> &str {
        "Something bad happened"
    }
}

impl fmt::Debug for ExpectErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
impl fmt::Display for ExpectErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

fn read_mode() -> Mode {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|v| v == "--expect_reset") {
        Mode::Reset
    } else {
        Mode::Test
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

    fn file_path(&self) -> std::path::PathBuf {
        env::current_dir()
            .unwrap()
            .join("expect")
            .join(&self.file_name)
    }

    fn expectation(&self) -> String {
        fs::read_to_string(self.file_path()).unwrap_or("".to_string())
    }

    // pub fn correct(&self) -> () {
    //     let expectation = self.expectation().unwrap();
    //     let result = self.buf.clone();
    //     assert_eq!(expectation, result)
    // }

    pub fn correct(&self) -> std::result::Result<(), ExpectErr> {
        let expectation = self.expectation();
        let result = self.buf.clone();
        let changeset = Changeset::new(&expectation, &result, "\"\"");
        match changeset.distance {
            0 => Ok(()),
            _ => Err(ExpectErr { changeset }),
        }
    }

    pub fn finish(&self) -> Result<(), Box<Error>> {
        match read_mode() {
            // Mode::Test => self.correct().map_err(|e| e.into()),
            Mode::Test => match self.correct() {
                Ok(()) => Ok(()),
                Err(changeset) => {
                    assert!(false, changeset);
                    Err(changeset).map_err(|e| e.into())
                }
            },
            Mode::Reset => self.write_to_file().map_err(|e| e.into()),
        }
    }
    pub fn write_to_file(&self) -> Result<(), io::Error> {
        let result = self.buf.clone();
        fs::write(self.file_path(), result)
    }
}

// I don't think you're allowed to panic within a Drop, so this doesn't work
// impl Drop for Expect {
//     fn drop(&mut self) {
//         match self.correct() {
//             Ok(()) => {}
//             Err(_e) => panic!("whoops"), //println!("whoops!"), // Err(e)),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_expect() {
        let _s = Expect::new("test1".to_string());
        // let _s = Expect::new(&"test1");
    }
    #[test]
    fn write_to_expect_incorrect() {
        let mut s = Expect::new("test1".to_string());
        s.push("x");
        assert!(s.correct().is_err());
    }
    #[test]
    fn write_to_expect_multiple_correct() {
        let mut s = Expect::new("test1".to_string());
        s.push("he");
        s.push("llo");
        assert!(s.correct().is_ok());
    }
    #[test]
    fn write_to_expect_correct() {
        let mut s = Expect::new("test1".to_string());
        s.push("hello");
        assert!(s.correct().is_ok());
    }
    #[test]
    fn write_to_file() {
        let mut s = Expect::new("test2".to_string());
        fs::remove_file(s.file_path()).ok();
        s.push("hello");
        s.write_to_file().unwrap();
        let contents = fs::read_to_string(s.file_path()).unwrap();
        assert!(contents == "hello")
    }
}

#[cfg(test)]
mod trials {
    use super::*;

    #[test]
    fn testing() {
        let mut s = Expect::new("testing".to_string());
        s.push("Hi Hi Hello");
        s.finish().ok();
    }
}
