// https://www.acmicpc.net/problem/18186

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

    let (n, b, mut c) = (sc.read::<usize>(), sc.read::<i64>(), sc.read::<i64>());
    if b < c {
        c = b
    }
    let mut v = [0; 1_000_002];
    (0..n).for_each(|i| v[i] = sc.read::<i64>());

    let mut min = 0;

    for i in 0..n {
        if v[i+1] > v[i+2] {
            let pair = v[i].min(v[i + 1] - v[i + 2]);
            v[i] -= pair;
            v[i + 1] -= pair;
            min += (b + c) * pair;

            let tri = v[i].min(v[i + 1].min(v[i + 2]));
            v[i] -= tri;
            v[i + 1] -= tri;
            v[i + 2] -= tri;
            min += (b + c + c) * tri;
        } else {
            let tri = v[i].min(v[i + 1].min(v[i + 2]));
            v[i] -= tri;
            v[i + 1] -= tri;
            v[i + 2] -= tri;
            min += (b + c + c) * tri;

            let pair = v[i].min(v[i + 1]);
            v[i] -= pair;
            v[i + 1] -= pair;
            min += (b + c) * pair;
        }
        min += b * v[i];
    }

    writeln!(output, "{min}")?;
    Ok(())
}