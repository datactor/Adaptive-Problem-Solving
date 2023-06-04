// https://www.acmicpc.net/problem/1557

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
    let mut k = sc.read::<i64>();
    let mut arr = [0; 1_000_001];
    arr[1] = 1;

    let (mut left, mut right) = (0, 2_000_000_000);

    for i in 1..=1_000_000 {
        if arr[i] != 0 {
            for j in ((i * 2)..=1_000_000).step_by(i) {
                arr[j] -= arr[i]
            }
        }
    }

    while left < right - 1 {
        let mid = (left + right) / 2;
        if is_sqare_free(mid, &arr) < k {
            left = mid;
        } else {
            right = mid;
        }
    }

    writeln!(output, "{}", right)?;

    Ok(())
}

fn is_square_free(n: i64, arr: &[i64; 1000001]) -> i64 {
    let mut p = 0;
    for i in 1..(n as f32).sqrt() as i64 + 1 {
        p += arr[i as usize] * (n / (i * i));
    } p
}