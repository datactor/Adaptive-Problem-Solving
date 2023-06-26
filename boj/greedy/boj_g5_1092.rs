// https://www.acmicpc.net/problem/1092

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
            .ok_or("Reached end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);

    let n = scanner.next::<usize>()?;
    let mut cranes = (0..n).map(|_| scanner.next().unwrap()).collect::<Vec<i32>>();

    let m = scanner.next::<usize>()?;
    let mut crates = (0..m).map(|_| scanner.next().unwrap()).collect::<Vec<i32>>();

    cranes.sort_unstable_by(|a, b| b.cmp(a));
    crates.sort_unstable_by(|a, b| b.cmp(a));

    if *crates.first().unwrap() > *cranes.first().unwrap() as i32 {
        write!(buf_writer, "{}", -1)?;
        return Ok(())
    }

    let mut times = 0;
    while !crates.is_empty() {
        for max_load in &cranes {
            for weight in 0..crates.len() {
                if max_load >= &crates[weight] {
                    crates.remove(weight);
                    break
                }
            }
        }
        times += 1;
    }

    write!(buf_writer, "{}", times)?;
    Ok(())
}