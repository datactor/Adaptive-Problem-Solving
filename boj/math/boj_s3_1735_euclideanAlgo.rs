// https://www.acmicpc.net/problem/1735

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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a }
    else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let mut a: (i64, i64) = (scanner.next()?, scanner.next()?);
    let mut b: (i64, i64) = (scanner.next()?, scanner.next()?);
    if a.1 < b.1 {
        std::mem::swap(&mut a, &mut b);
    };
    let numer = lcm(a.1, b.1);
    let denom  = (a.0 * numer / a.1) + (b.0 * numer / b.1);

    // get irreducible fraction factor
    let gcd = gcd(numer, denom);
    write!(buf_writer, "{} {}", denom/gcd, numer/gcd)?;

    Ok(())
}