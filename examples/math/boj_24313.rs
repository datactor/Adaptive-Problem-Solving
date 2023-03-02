// https://www.acmicpc.net/problem/24313

use std::{
    io::{self, prelude::*, BufWriter},
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

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let (a1, a0, c, n0) = (sc.read::<i32>(), sc.read::<i32>(), sc.read::<i32>(), sc.read::<i32>());

    writeln!(output, "{}", match a1 * n0 + a0 <= c * n0 && a1 <= c {
        true => 1,
        false => 0,
    })?;

    Ok(())
}