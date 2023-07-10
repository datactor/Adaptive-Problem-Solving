// https://www.acmicpc.net/problem/1003

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    
    let mut dp = Vec::with_capacity(41);
    let t = scanner.next::<usize>()?;
    dp.push((1, 0));
    dp.push((0, 1));
    for i in 2..41 {
        dp.push((dp[i - 1].0 + dp[i - 2].0, dp[i - 1].1 + dp[i - 2].1));
    }

    for _ in 0..t {
        let n = scanner.next::<usize>()?;
        writeln!(buf_writer, "{} {}", dp[n].0, dp[n].1)?;
    }

    Ok(())
}