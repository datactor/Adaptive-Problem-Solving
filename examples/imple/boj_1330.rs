// https://www.acmicpc.net/problem/1330

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
    io::stdin().read_line(&mut input)?;

    let mut sc = Scanner::new(&input);
    let x = sc.read::<i32>() - sc.read::<i32>();
    writeln!(output, "{}",
             match x {
                 0 => "==",
                 x if x > 0 => ">",
                 _ => "<",
             })?;
    Ok(())
}