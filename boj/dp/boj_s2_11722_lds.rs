// https://www.acmicpc.net/problem/11722

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

macro_rules! find_lds {
    ($reader:expr, $input:expr, $n:expr, $type:ty) => {
        {
            let mut vec = Vec::with_capacity($n);
            let mut dp = vec![1; $n];
            $input.clear();
            $reader.read_line(&mut $input)?;
            let mut iter = $input.split_ascii_whitespace();
            for i in 0..$n {
                let next = iter
                    .next().expect("no next iter")
                    .parse::<$type>().expect("Failed to parse");
                vec.push(next);
                for j in 0..i {
                    if vec[i] < vec[j] && dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                    }
                }
            }

            dp.iter().max().cloned()
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let Ok(n) = read_to_num!(read_buf, buf_to_string, usize) {
        if let Some(max) = find_lds!(read_buf, buf_to_string, n, usize) {
            writeln!(write_buf, "{}", max)?
        }
    }

    Ok(())
}