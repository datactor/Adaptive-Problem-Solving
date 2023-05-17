// https://www.acmicpc.net/problem/18111
// O(n*m + H^2)

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    str::{FromStr, SplitAsciiWhitespace},
    fmt,
    collections::BTreeMap,
};

trait Parser {
    fn read<T, E>(&mut self) -> T where T : FromStr<Err = E>,  E : fmt::Debug;
}

impl<'a> Parser for SplitAsciiWhitespace<'a> {
    fn read<T, E>(&mut self) -> T
        where
            T: FromStr<Err = E>,
            E: fmt::Debug,
    {
        match self.next() {
            Some(value) => value.parse().expect("Parse Error"),
            None => panic!("Unexpected EOF"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut min = usize::MAX;
    let mut highest = 0;

    let mut fst_line = buffer.split_ascii_whitespace();
    let (n, m, b): (usize, usize, usize) = (fst_line.read(), fst_line.read(), fst_line.read());
    let mut map = BTreeMap::new();

    for _ in 0..n {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        let mut iter = buffer.split_ascii_whitespace();
        for _ in 0..m {
            let a: usize = iter.read();
            let h = map.entry(a).or_insert(0);
            *h += 1;
        }
    }

    let range = *map.keys().next().unwrap()..=*map.keys().last().unwrap();
    for target in range.rev() {
        let mut times = 0;
        let mut pocket = b;
        for (k, v) in map.iter().rev() {
            if target == *k {
                continue
            } else if target < *k {
                let r = (k - target) * *v;
                pocket += r;
                times += r * 2;
            } else {
                let r = (target - k) * *v;
                if pocket < r {
                    times = usize::MAX;
                    break
                } else {
                    pocket -= r;
                    times += r;
                }
            }
        }
        if min > times {
            min = times;
            highest = target;
        }
    }

    write!(BufWriter::new(io::stdout().lock()), "{} {}", min, highest)?;
    Ok(())
}