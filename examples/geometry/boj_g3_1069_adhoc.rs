// https://www.acmicpc.net/problem/1069\
// O(1)

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input)?;

    let mut line = input.split_ascii_whitespace().map(|s| s.parse::<i32>()).flatten();

    let (x, y, d, t) = (line.next().unwrap(), line.next().unwrap(), line.next().unwrap(), line.next().unwrap());

    let dist = ((x.pow(2) + y.pow(2)) as f64).sqrt();

    writeln!(output, "{:.9}",
             if d <= t {
                 dist
             } else {
                 let mut min_time = dist;
                 let jump = (dist / d as f64).floor() as i32;

                 let remaining_dist = dist - jump as f64 * d as f64;

                 if jump == 0 {
                     min_time = min_time.min(f64::min(t as f64 + d as f64 - remaining_dist, 2.0 * t as f64));
                 } else {
                     min_time = min_time.min(f64::min(jump as f64 * t as f64 + remaining_dist, (jump as f64 + 1.0) * t as f64));
                 }
                 min_time
             }
    )?;

    Ok(())
}
