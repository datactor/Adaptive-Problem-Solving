// https://www.acmicpc.net/problem/11656

use std::{
    io::{self, Write, BufRead, BufWriter},
    cmp::Ordering,
};

fn main() -> io::Result<()> {
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut input = String::new();

    io::stdin().lock().read_line(&mut input)?;

    let mut v: Vec<String> = Vec::new();

    for i in 0..input.trim().len() {
        let tmp = input[i..].to_string();
        v.push(tmp);
    }

    v.sort_by(|a, b| {
        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for element in v {
        write!(write_buf, "{}", element)?;
    }

    Ok(())
}