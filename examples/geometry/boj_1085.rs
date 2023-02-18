use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    v[2] -= v[0];
    v[3] -= v[1];

    writeln!(output, "{}", v.iter().min().unwrap()).unwrap();
}