// https://www.acmicpc.net/problem/2170
// O(n lg n)

use std::{
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
    let mut lines = vec![(0, 0); n];

    for i in 0..n {
        let x = scanner.next::<i32>()?;
        let y = scanner.next::<i32>()?;
        lines[i] = (x, y);
    }
    lines.sort_unstable();

    // sweep line (left, right), (start, end)
    let mut sum = 0;
    let mut left = i32::MIN;
    let mut right = i32::MIN;

    // 겹치지 않는 구간을 모두 더하고, 겹치는 구간은 right를 늘려가며 합친다.
    for &(start, end) in &lines {
        if right < start {
            sum += right - left;
            left = start;
            right = end;
        } else {
            right = std::cmp::max(right, end);
        }
    }
    sum += right - left; // 최후에 겹치는 구간을 더해준다.
    write!(buf_writer, "{}", sum)?;
    Ok(())
}
