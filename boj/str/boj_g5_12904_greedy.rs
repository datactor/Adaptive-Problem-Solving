#![allow(non_snake_case)]

use std::{
    io::{self, Read, Write, BufReader, BufWriter},
};

macro_rules! Ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut write_buf = BufWriter::new(io::stdout().lock());
            let mut buf_to_string = String::new();

            read_buf.read_to_string(&mut buf_to_string)?;
            let mut words = Words::new(&buf_to_string);

            write!(write_buf, "{}", words.check())?;
            Ok(())
        }
    }
}

struct Words<'a> {
    s: &'a [u8],
    t: Vec<u8>,
}

impl<'a> Words<'a> {
    fn new(s: &'a str) -> Self {
        let mut iter = s.split_ascii_whitespace();
        Self {
            s: iter.next().expect("Failed to get s").as_bytes(),
            t: iter.next().expect("Failed to get t").as_bytes().to_vec(),
        }
    }

    fn check(&mut self) -> u8 {
        while self.t.len() > self.s.len() {
            if self.t.pop() == Some(b'B') {
                self.t.reverse()
            }
        };

        if self.s == self.t { 1 } else { 0 }
    }
}

fn main() -> io::Result<()> {
    Ok!(())
}