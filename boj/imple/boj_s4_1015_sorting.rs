// https://www.acmicpc.net/problem/1015

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.split_ascii_whitespace(),
        }
    }

    fn next<T>(&mut self) -> Result<T, Box<dyn Error>> 
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
    let n: usize = scanner.next()?;
    let mut b = vec![0; n];
    let a = (0..n)
        .map(|_| scanner.next::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut sorted_a = a.clone();
    sorted_a.sort_unstable();
    for i in 0..n {
        let pi = sorted_a.iter().position(|&x| x == a[i]).ok_or("Cannot found the position")?;
        b[i] = pi;
        sorted_a[pi] = -1;
    }

    for num in b {
        write!(buf_writer, "{} ", num)?;
    }

    Ok(())
}