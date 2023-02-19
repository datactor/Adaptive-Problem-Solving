// https://www.acmicpc.net/problem/10773
// O(N)

use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();

    let k = v.next().unwrap();
    let mut s = Vec::new();

    for _ in 0..k {
        let i = v.next().unwrap();
        match i {
            0 => {
                s.pop();
            }
            _ => {
                s.push(i);
            }
        }
    }
    writeln!(output, "{}", s.iter().sum::<usize>()).unwrap();
}
