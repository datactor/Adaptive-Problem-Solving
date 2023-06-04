// O(n^2)

use std::collections::HashSet;
use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let v = buffer.trim();
    let n = v.len();
    let mut hashset = HashSet::new();

    let mut elements = 1;
    let mut result = 0;
    while elements <= n {
        let mut i = 0;
        hashset.clear();
        while i + elements <= n {
            hashset.insert(&v[i..i + elements]);
            i += 1;
        }
        elements += 1;
        result += hashset.len();
    }
    writeln!(output, "{}", result);
}
