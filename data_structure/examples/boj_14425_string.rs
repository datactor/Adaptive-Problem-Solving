use std::io::{BufWriter, self, prelude::*};
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();
    let n = lines.next().unwrap().split_ascii_whitespace().next().unwrap().parse::<usize>().unwrap();
    let mut hashset = HashSet::with_capacity(n);
    for line in lines.by_ref().take(n) {
        hashset.insert(line);
    }
    let result = lines.fold(0, |s, l| s + hashset.contains(&l) as usize);
    writeln!(output, "{}", result).unwrap();
}