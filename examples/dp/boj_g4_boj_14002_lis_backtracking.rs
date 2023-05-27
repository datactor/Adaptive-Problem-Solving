// https://www.acmicpc.net/problem/14002

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_to_num {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $reader.read_line(&mut $input)?;
            $input.trim().parse::<usize>()
        }
    }
}

macro_rules! find_lis {
    ($reader:expr, $input:expr, $n:expr, $type:ty) => {
        {
            let mut vec = Vec::with_capacity($n);
            let mut dp = vec![vec![1]; $n];
            $input.clear();
            $reader.read_line(&mut $input)?;
            let mut iter = $input.split_ascii_whitespace();
            for i in 0..$n {
                let next = iter.next().expect("no next iter").parse::<$type>().expect("Failed to parse");
                vec.push(next);
                dp[i].push(next);
                for j in 0..i {
                    if vec[i] > vec[j] && dp[i][0] <= dp[j][0] {
                        dp[i] = dp[j].clone();
                        dp[i][0] += 1;
                        dp[i].push(vec[i])
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
        if let Some(vec) = find_lis!(read_buf, buf_to_string, n, usize) {
            if let Some(len) = vec.get(0) {
                write!(write_buf, "{}\n", len)?;
                for i in vec[1..].iter() {
                    write!(write_buf, "{} ", i)?
                }
            }
        }
    }

    Ok(())
}