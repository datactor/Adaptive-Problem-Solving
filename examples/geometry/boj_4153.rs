use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();

    while true {
        let mut v = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if v[0] == 0 && v[1] == 0 && v[2] == 0 {
            break;
        }
        v.sort();
        if v[0] * v[0] + v[1] * v[1] == v[2] * v[2] {
            writeln!(output, "right").unwrap()
        } else {
            writeln!(output, "wrong").unwrap()
        }
    }
}
