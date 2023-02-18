use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();

    while true {
        let mut v = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>()).flatten();

        let (a, b) = (v.next().unwrap(), v.next().unwrap());
        if (a, b) == (0, 0) {
            break
        } else if b % a == 0 && b >= a {
            writeln!(output, "factor").unwrap();
        } else if a % b == 0 && a >= b {
            writeln!(output, "multiple").unwrap();
        } else {
            writeln!(output, "neither").unwrap();
        }
    }
}