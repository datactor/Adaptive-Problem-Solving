// https://www.acmicpc.net/problem/9998

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
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn cal(yun: &Vec<i64>, don: &Vec<i64>, center: usize, offset: usize, i: i64, m: i64) -> i64 {
    (yun[center + offset] - (m + i)).abs() + (don[center + offset] - (m + i)).abs()
        + if offset > 0 {
            (yun[center - offset] - (m + i)).abs() + (don[center - offset] - (m + i)).abs()
        } else {
            0
        }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let center = n/2;
    let yun = (0..n).map(|_| scanner.next().unwrap()).collect::<Vec<i64>>();
    let don = (0..n).map(|_| scanner.next().unwrap()).collect::<Vec<i64>>();

    let mut a;
    let mut b;
    let mut res = 0;
    let mut l = 0;
    let mut m;
    let mut r = 1e12 as i64;
    while l <= r {
        m = (l + r) / 2;
        a = cal(&yun, &don, center, 0, 0, m);
        b = cal(&yun, &don, center, 0, 1, m);

        for i in 1..(n-center) {
            a += cal(&yun, &don, center, i, i as i64, m);
            b += cal(&yun, &don, center, i, (i + 1) as i64, m);
        }

        if a < b {
            res = a;
            r = m - 1;
        } else {
            res = b;
            l = m + 1;
        }
    }

    write!(buf_writer, "{}", res)?;
    Ok(())
}