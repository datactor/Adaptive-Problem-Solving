// https://www.acmicpc.net/problem/13411

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    cmp::Ordering,
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
        self.input.next().ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let mut vec = Vec::with_capacity(n);
    for i in 1..=n {
        let x = scanner.next::<f64>()?;
        let y = scanner.next::<f64>()?;
        let v = scanner.next::<f64>()?;
        let t = (x * x + y * y).sqrt() / v;
        vec.push((i, t));
    }
    
    vec.sort_unstable_by(|a, b| {
        match a.1.partial_cmp(&b.1) {
            Some(Ordering::Equal) => a.0.cmp(&b.0),
            other => other.unwrap(),
        }
    });
    for (i, _) in vec {
        writeln!(buf_writer, "{}", i)?;
    }
    Ok(())
}