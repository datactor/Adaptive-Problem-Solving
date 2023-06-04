// https://www.acmicpc.net/problem/1305
// Knuth, Morris, Prett Algorithm
// https://bowbowbow.tistory.com/6

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
    let l = sc.read::<usize>();
    let txt = sc.read::<String>();
    let pattern = txt.as_bytes();
    let mut table: Vec<usize> = vec![0; l+1];
    let mut pi = 0;
    for (i, b) in txt.as_bytes().iter().enumerate().skip(1) {
        while pi > 0 && &pattern[pi] != b {
            pi = table[pi - 1];
        }
        if &pattern[pi] == b {
            pi += 1;
            table[i] = pi;
        }
    }
    writeln!(output, "{}", l - table[l-1])?;
    Ok(())
}