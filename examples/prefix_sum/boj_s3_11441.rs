// https://www.acmicpc.net/problem/11441

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! get_sums {
    ($reader:expr, $writer:expr, $type:ty) => {
        {
            let mut line = || $reader.next().expect("Failed to get next line").expect("Failed to read line");
            let n = line().parse::<usize>().expect("Failed to parse");

            let mut table = vec![0; n+1];

            let parts_line = line();
            let mut iter = parts_line.split_ascii_whitespace();
            for i in 1..=n {
                let parts = iter.next().expect("Failed to get next iter").parse::<$type>().expect("Failed to parse");
                table[i] += parts + table[i-1];
            }

            let m = line().parse::<usize>().expect("Failed to parse");
            for _ in 0..m {
                let range_line = line();
                let mut range = range_line.split_ascii_whitespace();
                let l = range.next().expect("Failed to read next iter").parse::<usize>().expect("Failed to parse");
                let r = range.next().expect("Failed to read next iter").parse::<usize>().expect("Failed to parse");
                write!($writer, "{}\n", table[r] - table[l-1])?
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf_to_lines = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());
    get_sums!(read_buf_to_lines, write_buf, i32);
    Ok(())
}