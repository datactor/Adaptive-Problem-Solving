// https://www.acmicpc.net/problem/10828
// O(N)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let mut v = Vec::new();
    let mut n = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut line = lines.next().unwrap().split_ascii_whitespace();
        match line.next().unwrap() {
            "push" => v.push(line.next().unwrap().parse::<i32>().unwrap()),
            "pop" => writeln!(output, "{}", v.pop().unwrap_or(-1)).unwrap(),
            "size" => writeln!(output, "{}", v.len()).unwrap(),
            "empty" => if v.is_empty() { writeln!(output, "1").unwrap() } else { writeln!(output, "0").unwrap() },
            "top" => if v.is_empty() { writeln!(output, "-1").unwrap() } else { writeln!(output, "{}", v[v.len()-1]).unwrap() }
            _ => {},
        }
    }

    Ok(())
}