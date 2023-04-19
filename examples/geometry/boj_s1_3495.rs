// https://www.acmicpc.net/problem/3495

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut buffer = String::new();
    input.read_line(&mut buffer)?;
    let (h, w) = buffer.trim().split_once(' ').unwrap();
    let (h, w) = (h.parse::<usize>().unwrap(), w.parse::<usize>().unwrap());

    let mut sum = 0;
    for _ in 0..h {
        buffer.clear();
        input.read_line(&mut buffer)?;
        let chars = buffer.as_bytes();
        let mut open = false;
        let mut line_sum = 0;
        for j in 0..w {
            let c = chars[j];
            if c != b'.' {
                if !open {
                    open = true;
                } else {
                    open = false;
                    line_sum += 1;
                }
            } else if open {
                line_sum += 1;
            }
        }
        sum += line_sum;
    }
    writeln!(output, "{}", sum)?;
    Ok(())
}