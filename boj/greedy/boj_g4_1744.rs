// https://www.acmicpc.net/problem/1744

use std::{
    io::{self, Write, Read, BufWriter},
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
        self
            .input
            .next()
            .ok_or("Reached end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut sc = Scanner::new(&buffer);
    let mut vec = sc.input.skip(1).map(|s| s.parse::<i32>()).flatten().collect::<Vec<i32>>();
    vec.sort();

    let mut pos = Vec::new();
    let mut zero = Vec::new();
    let mut neg = Vec::new();

    let mut sum = 0;
    while let Some(num) = vec.pop() {
        match num.signum() {
            1 => pos.push(num),
            0 => zero.push(num),
            _ => neg.push(num),
        }

        if pos.len() == 2 && pos[0] != 1 && pos[1] != 1 {
            sum += pos[0] * pos[1];
            pos.clear();
        }
    }

    while let Some(po) = pos.pop() {
        sum += po;
    }

    let mut to_mul = 0;
    while let Some(ne) = neg.pop() {
        if to_mul != 0 {
            sum += to_mul * ne;
            to_mul = 0;
        } else {
            to_mul = ne;
        }
    }

    if zero.is_empty() { sum += to_mul }

    write!(buf_writer, "{}", sum)?;

    Ok(())
}