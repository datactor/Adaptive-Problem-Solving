// https://www.acmicpc.net/problem/18185

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

    let n = sc.read::<usize>();
    let mut v = [0; 10_002];
    (0..n).for_each(|i| v[i] = sc.read::<usize>());

    let mut min = 0;

    for i in 0..n {
        if v.get(i + 1).unwrap_or(&0) > v.get(i + 2).unwrap_or(&0) {
            let pair = v[i].min(v[i + 1] - v[i + 2]);
            min += 5 * pair;
            v[i] -= pair;
            v[i + 1] -= pair;

            let trio = v[i].min(v[i + 1].min(v[i + 2]));
            min += 7 * trio;
            v[i] -= trio;
            v[i + 1] -= trio;
            v[i + 2] -= trio;
        } else {
            let trio = v[i].min(v[i + 1].min(v[i + 2]));
            min += 7 * trio;
            v[i] -= trio;
            v[i + 1] -= trio;
            v[i + 2] -= trio;

            let pair = v[i].min(v[i + 1]);
            min += 5 * pair;
            v[i] -= pair;
            v[i + 1] -= pair;
        }

        min += 3 * v[i];
    }

    writeln!(output, "{}", min)?;

    Ok(())
}