// https://www.acmicpc.net/problem/1715
// Error propagate

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    collections::BinaryHeap,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input:s.split_ascii_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> Result<T, Box<dyn Error>>
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.input
            .next()
            .ok_or("Reached end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let mut hq = BinaryHeap::with_capacity(n);

    for _ in 0..n {
        let deck = -scanner.next::<i32>()?;
        hq.push(deck);
    }

    let mut sum = 0;
    let mut tmp = 0;
    while let Some(min) = hq.pop() {
        if tmp == 0 {
            tmp += min;
        } else {
            tmp += min;
            sum += tmp;
            hq.push(tmp);
            tmp = 0;
        }
    }
    write!(write_buf, "{}", -sum)?;

    Ok(())
}