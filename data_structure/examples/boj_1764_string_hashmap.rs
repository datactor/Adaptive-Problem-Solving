// O(N logN)

use std::io::{self, prelude::*, BufWriter};
use std::collections::HashSet;

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace();
    let n = first_line.next().unwrap().parse::<usize>().unwrap();
    let m = first_line.next().unwrap().parse::<usize>().unwrap();

    let mut hashset = HashSet::with_capacity(n);
    for _ in 0..n {
        let a = lines.next().unwrap().trim();
        hashset.insert(a);
    }

    let mut result = Vec::new();

    for _ in 0..m {
        let a = lines.next().unwrap().trim();
        match hashset.get(a) {
            None => {},
            _ => result.push(a),
        }
    }

    writeln!(output, "{}", result.len()).unwrap();

    // O(N logN)
    result.sort();
    for i in result {
        writeln!(output, "{}", i).unwrap();
    }
}