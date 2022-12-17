use std::io::{self, prelude::*, BufWriter};
use std::collections::HashSet;

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();

    let mut x: HashSet<usize> = HashSet::new();
    let mut y = HashSet::new();
    for _ in 0..3 {
        let mut tmp = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>()).flatten();
        let a = tmp.next().unwrap();
        let b = tmp.next().unwrap();

        if x.contains(&a) {
            x.remove(&a);
        } else {
            x.insert(a);
        }

        if y.contains(&b) {
            y.remove(&b);
        } else {
            y.insert(b);
        };
    }

    writeln!(output, "{} {}", x.iter().next().unwrap(), y.iter().next().unwrap()).unwrap();
}