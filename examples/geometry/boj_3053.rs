use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let r = buffer.split_ascii_whitespace().next().unwrap().parse::<usize>().unwrap();

    writeln!(output, "{:0.6}\n{:0.6}", std::f64::consts::PI*(r*r) as f64, (r*r*2) as f64).unwrap();
}