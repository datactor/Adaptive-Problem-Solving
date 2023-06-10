// https://www.acmicpc.net/problem/5582

#![allow(non_snake_case)]

use std::{
    io::{self, Write, Read, BufReader, BufWriter},
};

macro_rules! Ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut write_buf = BufWriter::new(io::stdout().lock());
            let mut buf_to_string = String::new();

            read_buf.read_to_string(&mut buf_to_string)?;
            let words = Words::new(&buf_to_string);

            write!(write_buf, "{}", words.find_len())?;

            Ok(())
        }
    }
}

struct Words<'a> {
    s: &'a [u8],
    t: &'a [u8],
}

impl<'a> Words<'a> {
    fn new(s: &'a str) -> Self {
        let mut iter = s.split_ascii_whitespace();
        Self {
            s: iter.next().expect("Failed to get next iter").as_bytes(),
            t: iter.next().expect("Failed to get next iter").as_bytes(),
        }
    }

    fn find_len(&self) -> i32 {
        let mut ans = 0;
        let s_len = self.s.len();
        let t_len = self.t.len();
        let mut dp = vec![0; t_len +1];

        for i in 1..s_len +1 {
            let mut tmp = vec![0; t_len +1];
            for j in 1..t_len +1 {
                if self.s[i-1] == self.t[j-1] {
                    tmp[j] = dp[j-1] + 1;
                    ans = std::cmp::max(tmp[j], ans);
                }
            }
            dp = tmp
        }

        ans
    }
}

fn main() -> io::Result<()> {
    Ok!(())
}