use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut v = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        v.sort();

        let m = v[0] * v[1];
        while v[1] % v[0] != 0 {
            let t = v[0];
            v[0] = v[1] % v[0];
            v[1] = t;
        }
        writeln!(output, "{}", m / v[0]).unwrap();
    }
}