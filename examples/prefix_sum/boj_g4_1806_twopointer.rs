// https://www.acmicpc.net/problem/1806
// O(n)

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_to_nums {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $reader.read_line(&mut $input)?;
            let ns = $input.trim().split_once(' ').expect("no n & s");
            let (n, s) = (ns.0, ns.1);
            (n.parse::<$type>(), s.parse::<i32>())
        }
    }
}

macro_rules! find_len {
    ($reader:expr, $input:expr, $n:expr, $s:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let table = $input
                            .split_ascii_whitespace()
                            .map(|s| s.parse::<$type>().expect("Failed to parse"))
                            .collect::<Vec<$type>>();
            let mut min_len = i32::MAX;
            let (mut left, mut right, mut sum) = (0, 0, 0);

            loop {
                if sum >= $s {
                    min_len = min_len.min(right - left);
                    sum -= table[left as usize];
                    left += 1;
                } else if right == $n as i32 {
                    break;
                } else {
                    sum += table[right as usize];
                    right += 1;
                }
            }

            if min_len == i32::MAX {
                0
            } else {
                min_len
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let (Ok(n), Ok(s)) = read_to_nums!(read_buf, buf_to_string, usize) {
        writeln!(write_buf, "{}", find_len!(read_buf, buf_to_string, n, s, i32))?;
    }
    Ok(())
}