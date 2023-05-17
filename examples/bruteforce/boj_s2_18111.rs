// https://www.acmicpc.net/problem/18111
// O(H^2)

use std::{
    io::{self, BufRead, BufReader},
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
    // let mut x = [0; 257];

    let mut min = usize::MAX;
    let mut highst = 0;

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
            // x[a] += 1;
        }
    }

    let range = *map.keys().min().unwrap()..*map.keys().max().unwrap() + 1;
    for target in range.rev() {
        let mut times = 0;
        let mut pocket = b;
        for (k, v) in map.iter().rev() {
            if target == *k {
                continue
            } else if target < *k {
                let rem = (k - target) * *v;
                pocket += rem;
                times += rem * 2;
            } else {
                let rem = (target - k) * *v;
                if pocket < rem {
                    times = usize::MAX;
                    break
                } else {
                    pocket -= rem;
                    times += rem;
                }
            }
        }
        if min > times {
            min = times;
            highst = target;
        }
    }

    print!("{} {}", min, highst);

    Ok(())
}