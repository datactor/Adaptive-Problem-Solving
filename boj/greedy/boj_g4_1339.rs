// https://www.acmicpc.net/problem/1339

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    error::Error,
    collections::{HashMap, BinaryHeap},
};

macro_rules! read {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.trim()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    let n = read!(read_buf, buf_to_string).parse::<usize>()?;
    let mut total = BinaryHeap::with_capacity(10);
    let mut hash = HashMap::with_capacity(10);
    for _ in 0..n {
        let v = read!(read_buf, buf_to_string).as_bytes().to_vec();
        for (i, b_idx) in (0..v.len()).rev().enumerate() {
            let digit: u32 = 10u32.pow(i as u32);
            hash.entry(v[b_idx]).or_insert(vec![]).push(digit);
        }
    }

    for (_, v) in hash {
        total.push(v.iter().sum::<u32>());
    }

    let mut largest: i8 = 9;
    let mut ans = 0;
    while let Some(max) = total.pop() {
        ans += largest as u32 * max;
        largest -= 1;
    }

    write!(write_buf, "{}", ans)?;

    Ok(())
}