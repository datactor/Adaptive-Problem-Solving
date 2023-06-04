// https://www.acmicpc.net/problem/1806
// O(n)

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_nums {
    ($reader:expr, $type:ty) => {
        {
            let line = $reader.next().expect("Failed to get next line").expect("Failed to read the line");
            line
                .split_ascii_whitespace() // ascii 문자열 범위 내에서만 처리하기 때문에 범위가 한정되어있어 일반적으로 더 빠름
                .map(|s| s.parse::<$type>().expect(&format!("Failed to parse '{}' into a '{}'", s, stringify!($type))))
                .collect::<Vec<$type>>()
        }
    }
}

macro_rules! find_len {
    ($table:expr, $n:expr, $s:expr, $type:ty) => {
        {
            let mut min_len = i32::MAX;
            let (mut left, mut right, mut sum) = (0, 0, 0);

            loop {
                if sum >= $s as i32 {
                    min_len = min_len.min(right - left);
                    sum -= $table[left as usize];
                    left += 1;
                } else if right == $n as i32 {
                    break;
                } else {
                    sum += $table[right as usize];
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
    let mut read_buf = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());

    let params = read_nums!(read_buf, usize);
    let vec = read_nums!(read_buf, i32);
    writeln!(write_buf, "{}", find_len!(vec, params[0], params[1], i32))?;

    Ok(())
}