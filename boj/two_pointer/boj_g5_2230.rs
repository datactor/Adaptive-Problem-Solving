// https://www.acmicpc.net/problem/2230

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_nums {
    ($reader:expr, $type:ty) => {
        {
            $reader
                .next()
                .expect("Failed to get next line")
                .expect("Failed to read line")
                .split_ascii_whitespace()
                .map(|s| s.parse::<$type>().expect("Failed to parse"))
                .collect::<Vec<$type>>()
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());

    let nm = read_nums!(read_buf, usize);
    let mut seq = (0..nm[0]).map(|_| {
        read_buf
            .next()
            .expect("Failed to get next line")
            .expect("Failed to read next line")
            .trim()
            .parse::<i32>()
            .expect("Failed to parse")
    }).collect::<Vec<i32>>();
    seq.sort();

    let (mut s, mut e) = (0, 1);
    let mut min = i32::MAX;

    while s <= e && e < nm[0] {
        let tmp = seq[e] - seq[s];
        if tmp == nm[1] as i32 {
            min = tmp;
            break
        }
        if tmp < nm[1] as i32{
            e += 1;
            continue
        }
        s += 1;
        min = std::cmp::min(min, tmp);
    }
    writeln!(write_buf, "{}", min)?;

    Ok(())
}