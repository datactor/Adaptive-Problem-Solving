// https://www.acmicpc.net/problem/9012
// O(N)

use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let mut n = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut v: Vec<char> = Vec::new();
        let mut line = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.chars())
            .flatten();
        let mut ans = "NO";
        for i in line {
            match i {
                '(' => v.push(i),
                _ => {
                    if v.pop() != Some('(') {
                        v.push(')');
                        break;
                    }
                }
            }
        }
        if v.is_empty() {
            ans = "YES"
        }
        writeln!(output, "{}", ans).unwrap();
    }

    Ok(())
}
