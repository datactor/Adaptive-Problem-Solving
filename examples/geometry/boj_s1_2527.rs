// https://www.acmicpc.net/problem/2527

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock();
    let mut output = BufWriter::new(io::stdout().lock());

    for _ in 0..4 {
        let mut line = String::new();
        input.read_line(&mut line)?;
        let v = line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

        let (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) = (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]);

        writeln!(output, "{}", if ax2 < bx1 || bx2 < ax1 || ay1 > by2 || ay2 < by1 {
            "d"
        } else if ax1 == bx2 || bx1 == ax2 {
            if ay2 == by1 || by2 == ay1 {
                "c"
            } else {
                "b"
            }
        } else if ay2 == by1 || by2 == ay1 {
            "b"
        } else {
            "a"
        })?;
    }

    Ok(())
}
