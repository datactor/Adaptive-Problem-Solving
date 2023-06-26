// https://www.acmicpc.net/problem/1461

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
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let (n, m) = (scanner.next::<usize>()?, scanner.next::<usize>()?);

    // let mut pos = (0..n).map(|_| scanner.next().unwrap()).collect::<Vec<i32>>();
    //
    // let mut neg = pos.iter().filter_map(|&x| (x < 0).then(|| x)).collect::<Vec<i32>>();
    // pos.retain(|&x| x > 0);
    let mut pos = Vec::new();
    let mut neg = Vec::new();

    for _ in 0..n {
        let n = scanner.next::<i32>()?;
        if n < 0 { neg.push(n) } else { pos.push(n) }
    }

    neg.sort_unstable();
    pos.sort_unstable_by(|a, b| b.cmp(a));

    neg.push(0);
    pos.push(0);

    let mut step = pos.chunks(m as usize)
        .map(|chunk| chunk[0]).sum::<i32>() - neg.chunks(m as usize)
        .map(|chunk| chunk[0]).sum::<i32>();

    step <<= 1;
    step -= std::cmp::max(pos[0], -neg[0]);

    write!(buf_writer, "{}", step)?;
    Ok(())
}