// https://www.acmicpc.net/problem/13904

use std::{
    io::{self, Write, Read, BufWriter, ErrorKind},
    error::Error,
    collections::BinaryHeap,
    str::FromStr,
};

struct Scanner<T: FromStr + Copy> {
    input: Vec<T>,
    index: usize,
}


impl<'a, T: FromStr + Copy> Scanner<T>
    where T::Err: std::fmt::Debug
{
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace()
                .filter_map(|word| word.parse().ok())
                .collect(),
            index: 0,
        }
    }

    fn next(&mut self) -> Result<(T, T), Box<dyn Error>> {
        if self.index + 1 < self.input.len() {
            let chunk = (self.input[self.index], self.input[self.index + 1]);
            self.index += 2;
            Ok(chunk)
        } else {
            Err(Box::new(io::Error::new(ErrorKind::Other, "Reached end of input")))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();
    io::stdin().read_to_string(&mut buf_to_string)?;

    let mut scanner: Scanner<i32> = Scanner::new(&buf_to_string);
    scanner.input.remove(0);

    let mut hq = BinaryHeap::new();
    let mut last_day = 0;

    while let Ok(chunk) = scanner.next() {
        hq.push((chunk.1, chunk.0));
        if last_day < chunk.0 { last_day = chunk.0 }
    }

    let mut assigned = vec![false; last_day as usize + 1];

    let mut score = 0;
    while let Some((w, d)) = hq.pop() {
        for i in (1..=d as usize).rev() {
            if !assigned[i] {
                assigned[i] = true;
                score += w;
                break;
            }
        }
    }

    write!(write_buf, "{}", score)?;

    Ok(())
}