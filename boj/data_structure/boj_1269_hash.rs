// O(N)

use std::collections::HashSet;
use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut fst_set = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<HashSet<_>>();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut snd_set = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<HashSet<_>>();

    let mut sub = 0;
    for i in &fst_set {
        match snd_set.get(&i) {
            None => sub += 1,
            _ => {}
        }
    }

    for i in &snd_set {
        match fst_set.get(&i) {
            None => sub += 1,
            _ => {}
        }
    }
    writeln!(output, "{}", sub);
}
