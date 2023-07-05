// https://www.acmicpc.net/problem/10811

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn next<T> (&mut self) -> Result<T, Box<dyn Error>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
    {
        self.input.next()
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let m = scanner.next::<usize>()?;

    let mut vec = (1..=n).map(|i| i).collect::<Vec<usize>>();
    for _ in 0..m {
        let i = scanner.next::<usize>()? - 1;
        let j = scanner.next::<usize>()?;
        vec[i..j].reverse();
    }

    for i in vec {
        write!(buf_writer, "{} ", i)?;
    }
    Ok(())
}