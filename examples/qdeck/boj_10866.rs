// https://www.acmicpc.net/problem/10866

use std::{
    collections::VecDeque,
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut v = VecDeque::new();
    for _ in 0..n {
        let mut line = lines.next().unwrap().split_ascii_whitespace();
        match line.next().unwrap() {
            "push_front" => v.push_front(line.next().unwrap().parse::<i32>().unwrap()),
            "push_back" => v.push_back(line.next().unwrap().parse::<i32>().unwrap()),
            "pop_front" => writeln!(output, "{}", v.pop_front().unwrap_or(-1))?,
            "pop_back" => writeln!(output, "{}", v.pop_back().unwrap_or(-1))?,
            "size" => writeln!(output, "{}", v.len())?,
            "empty" => writeln!(
                output,
                "{}",
                match v.is_empty() {
                    true => 1,
                    _ => 0,
                }
            )?,
            "front" => writeln!(output, "{}", v.front().unwrap_or(&-1))?,
            _ => writeln!(output, "{}", v.back().unwrap_or(&-1))?,
        }
    }
    Ok(())
}
