// https://www.acmicpc.net/problem/11055

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_to_num {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $reader.read_line(&mut $input)?;
            $input.trim().parse::<$type>()
        }
    }
}

macro_rules! read_to_vec {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.split_ascii_whitespace().map(|s| s.parse::<$type>().expect("Failed to parse")).collect::<Vec<$type>>()
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let Ok(n) = read_to_num!(read_buf, buf_to_string, usize) {
        let vec = read_to_vec!(read_buf, buf_to_string, usize);
        let mut dp = vec.clone();

        let mut max = 0;

        for i in 0..n {
            for j in 0..i {
                if vec[i] > vec[j] && dp[i] < dp[j] + vec[i] {
                    dp[i] = dp[j] + vec[i];
                }
            }
            max = max.max(dp[i]);
        }

        writeln!(write_buf, "{}", max)?;
    }

    Ok(())
}