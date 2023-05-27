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

macro_rules! find_lis {
    ($reader:expr, $writer:expr, $input:expr, $n:expr, $type:ty) => {
        {
            let mut vec = Vec::with_capacity($n);
            let mut dp = Vec::with_capacity($n);
            let mut max = 0;
            let mut write_buf = BufWriter::new(io::stdout().lock());

            $input.clear();
            $reader.read_line(&mut $input)?;
            let mut iter = $input.split_ascii_whitespace();
            for i in 0..$n {
                let next = iter
                .next().expect("no next iter")
                .parse::<$type>().expect("Failed to parse");
                vec.push(next);
                dp.push(next);
                for j in 0..i {
                    if vec[i] > vec[j] && dp[i] < dp[j] + vec[i] {
                        dp[i] = dp[j] + vec[i];
                    }
                }
                max = max.max(dp[i]);
            }
            writeln!($writer, "{}", max)
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());

    let mut buf_to_string = String::new();

    if let Ok(n) = read_to_num!(read_buf, buf_to_string, usize) {
        find_lis!(read_buf, write_buf, buf_to_string, n, usize)?
    }

    Ok(())
}