use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut v = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    v.sort();

    let x = v[0] * v[1];

    while v[1] % v[0] != 0 {
        let tmp = v[0];
        v[0] = v[1] % v[0];
        v[1] = tmp;
    }
    writeln!(output, "{}\n{}", v[0], x / v[0]).unwrap();
}
