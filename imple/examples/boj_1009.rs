// https://www.acmicpc.net/problem/1009
// O(n)

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

    let t = sc.read::<usize>();
    for _ in 0..t {
        let (mut a, mut b) = (sc.read::<u32>(), sc.read::<u32>());
        a %= 10;
        b %= 4;
        b += 4;
        let result = match a {
            0 => 10,
            1 => 1,
            5 => 5,
            6 => 6,
            _ => a.pow(b) % 10,
        };
        writeln!(output, "{}", result)?;
    }

    Ok(())
}