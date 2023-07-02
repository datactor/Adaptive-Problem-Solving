// https://www.acmicpc.net/problem/13324

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
    let n = scanner.next::<usize>()?;

    let mut ans = vec![0; n];
    let mut hq = BinaryHeap::new();

    for i in 0..n {
        let required_movement = scanner.next::<i32>()? - i as i32;

        hq.push(required_movement);
        hq.push(required_movement);
        hq.pop();
        ans[i] = *hq.peek().ok_or("Empty Heap")?;
    }

    for i in (1..n).rev() {
        if ans[i-1] > ans[i] {
            ans[i-1] = ans[i];
        }
    }

    for i in 0..n {
        writeln!(buf_writer, "{}", ans[i] + i as i32)?;
    }
    Ok(())
}