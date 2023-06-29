// https://www.acmicpc.net/problem/9063

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

    fn read<T> (&mut self) -> Result<T, Box<dyn Error>>
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
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut scanner = Scanner::new(&buffer);

    let n = scanner.read::<usize>()?;
    let ans = if n < 2 { 0 }
    else {
        let mut xmn = i32::MAX;
        let mut xmx = i32::MIN;
        let mut ymn = xmn;
        let mut ymx = xmx;
        for _ in 0..n {
            let x = scanner.read::<i32>()?;
            let y = scanner.read::<i32>()?;

            if x < xmn { xmn = x }
            if x > xmx { xmx = x }

            if y < ymn { ymn = y }
            if y > ymx { ymx = y }
        }
        (xmx - xmn) * (ymx - ymn)
    };

    write!(buf_writer, "{}", ans)?;
    Ok(())
}