// https://www.acmicpc.net/problem/2212

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
            .ok_or("Reached end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let mut n = scanner.next::<usize>()?;
    let k = scanner.next::<usize>()?;

    let mut sensors = (0..n).map(|_| scanner.next::<i32>().unwrap()).collect::<Vec<i32>>();
    sensors.sort_unstable();
    sensors.dedup();

    n = sensors.len();
    if n <= k {
        write!(buf_writer, "0")?;
        return Ok(())
    }

    let mut dist = Vec::with_capacity(n);
    for i in 1..n {
        dist.push(sensors[i] - sensors[i-1])
    }
    dist.sort_unstable();

    for _ in 0..k-1 {
        dist.pop();
    }
    write!(buf_writer, "{}", dist.iter().sum::<i32>())?;

    Ok(())
}