// https://www.acmicpc.net/problem/2437

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
            .ok_or("Reached end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

macro_rules! ok {
    (()) => {
        {
            let mut buf_writer = BufWriter::new(io::stdout().lock());
            let mut buffer = String::new();
            io::stdin().lock().read_to_string(&mut buffer)?;

            let mut scanner = Scanner::new(&buffer);
            let n = scanner.next::<usize>()?;
            let mut weight = (0..n).map(|_| scanner.next::<i32>().unwrap()).collect::<Vec<i32>>();
            weight.sort_unstable();

            let mut mn = 0;
            for w in weight {
                if w <= mn + 1 {
                    mn += w;
                } else {
                    break
                }
            }
            write!(buf_writer, "{}", mn+1)?;

            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    ok!(())
}