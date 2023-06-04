// https://www.acmicpc.net/problem/17609

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_line {
    ($reader:expr) => {
        {
            $reader.next().expect("Failed to get next line").expect("Failed to read line")
        }
    }
}

fn palindrome(vec: &[u8], mut s: usize, mut e: usize, mut cnt: usize) -> usize {
    while cnt < 2 && s < e {
        if vec[s] == vec[e] {
            s += 1;
            e -= 1;
        } else if cnt == 0 {
            let l = palindrome(vec, s+1, e, cnt+1);
            let r = palindrome(vec, s, e-1, cnt+1);
            return std::cmp::min(l, r)
        } else {
            cnt += 1;
        }
    }
    cnt
}

fn main() -> io::Result<()> {
    let mut read_buf_to_lines = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());

    let n = read_line!(read_buf_to_lines).parse::<usize>().expect("Failed to parse");
    for _ in 0..n {
        let string = read_line!(read_buf_to_lines);
        let len = string.len();
        let bytes = string.as_bytes();

        let (s, e) = (0, len-1);
        let cnt = 0;

        writeln!(write_buf, "{}", palindrome(bytes, s, e, cnt))?;
    }

    Ok(())
}