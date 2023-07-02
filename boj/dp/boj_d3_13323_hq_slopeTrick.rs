// https://www.acmicpc.net/problem/13323
// ref: https://velog.io/@idwooin/Slope-Trick

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    collections::BinaryHeap,
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
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<i32>()?;

    let mut res = 0;
    let mut hq = BinaryHeap::with_capacity(n as usize);
    for i in 0..n {
        let num = scanner.next::<i32>()? - i;

        if !hq.is_empty() && *hq.peek().unwrap() > num {
            hq.push(num);
            res += (hq.pop().unwrap() - num) as i64;
            hq.push(num);
        } else {
            hq.push(num);
        }
    }

    write!(buf_writer, "{}", res)?;
    Ok(())
}