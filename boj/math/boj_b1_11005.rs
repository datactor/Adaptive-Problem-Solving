// https://www.acmicpc.net/problem/11005

use std::{
    error::Error,
    io::{self, BufWriter, Write},
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
        self.input
            .next()
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let mut n = scanner.next::<usize>()?;
    let b = scanner.next::<usize>()?;

    let change: Vec<char> = (0..36)
        .map(|i| {
            if i < 10 {
                ('0' as u8 + i as u8) as char
            } else {
                ('A' as u8 - 10 + i as u8) as char
            }
        })
        .collect();

    if n == 0 {
        write!(writer, "0")?;
    } else {
        let mut ans: Vec<char> = Vec::new();

        while n != 0 {
            ans.push(change[(n % b) as usize]);
            n /= b;
        }

        ans.reverse();
        for ch in ans {
            write!(writer, "{}", ch)?;
        }
    }
    Ok(())
}
