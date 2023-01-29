// https://www.acmicpc.net/problem/1927

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::BinaryHeap,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace().skip(1);

    let mut heapq = BinaryHeap::new();

    for i in input {
        let x = i.parse::<i32>().unwrap();
        match x {
            0 => writeln!(output, "{}", heapq.pop().unwrap_or(0) * -1)?,
            _ => heapq.push(x*-1),
        }
    }

    Ok(())
}