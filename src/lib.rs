#![allow(unused_imports)]
use std::fmt;
use std::io;
use std::io::Write;
use std::str;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Snapshot {
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

impl Write for Snapshot {
    fn write(&mut self, text: &[u8]) -> io::Result<usize> {
        let t = str::from_utf8(text).unwrap();
        self.buf.push_str(t);
        Ok(text.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Snapshot {
    pub fn new(file_name: String) -> Snapshot {
        let buf = String::new();
        Snapshot { file_name, buf }
    }
    pub fn finish(&self) -> std::result::Result<(), ExpectErr> {
        let f = File::open(&self.file_name);
        let mut expectation = String::new();
        match f {
            Ok(mut i) => i.read_to_string(&mut expectation).unwrap(),
            Err(_i) => 0,
        };
        let result = self.buf.clone();
        if result == expectation {
            Ok(())
        } else {
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
    fn create_snapshot() {
        let s = Snapshot::new(String::from("test"));
    }
    #[test]
    fn write_to_snapshot() {
        let mut s = Snapshot::new(String::from("test"));
        s.write(&"x".as_bytes()).unwrap();
        assert!(s.finish().is_err());
    }
}
