// https://www.acmicpc.net/problem/7785

use std::{
    collections::BTreeSet,
    error::Error,
    io::{self, BufWriter, Read, Write},
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
            .ok_or("Reached out end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;

    let mut set: BTreeSet<Vec<u8>> = BTreeSet::new();
    for _ in 0..n {
        let name = scanner.next::<String>()?;
        let x = name.as_bytes();
        if let Some(_) = set.get(x) {
            set.remove(&x.to_owned());
        } else {
            set.insert(x.to_owned());
        }
        scanner.next::<String>()?;
    }

    for name in set.iter().rev() {
        buf_writer.write_all(name)?;
        buf_writer.write_all(b"\n")?;
    }
    Ok(())
}
