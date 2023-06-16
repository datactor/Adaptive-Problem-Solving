// https://www.acmicpc.net/problem/10266

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

const N: usize = 360_000;

macro_rules! read {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.split_ascii_whitespace()
        }
    }
}

macro_rules! ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut write_buf = BufWriter::new(io::stdout().lock());
            let mut buf_to_string = String::new();

            read!(read_buf, buf_to_string);
            let mut text = vec![false; 2 * N];
            let mut pattern = vec![false; N];
            let mut pi = vec![0; N];

            let tmp = read!(read_buf, buf_to_string);
            for s in tmp {
                let t = s.parse::<usize>().expect("Failed to parse");
                text[t] = true;
                text[t + N] = true;
            }
            let tmp = read!(read_buf, buf_to_string);
            for s in tmp {
                let t = s.parse::<usize>().expect("Failed to parse");
                pattern[t] = true;
            }

            get_pi(&pattern, &mut pi);
            write!(write_buf, "{}", if kmp(&text, &pattern, &pi) {
                "possible"
            } else {
                "impossible"
            })?;

            Ok(())
        }
    }
}

fn get_pi(pattern: &Vec<bool>, pi: &mut Vec<usize>) {
    let mut j = 0;
    for i in 1..N {
        while j > 0 && pattern[i] != pattern[j] {
            j = pi[j - 1];
        }
        if pattern[i] == pattern[j] {
            j += 1;
            pi[i] = j;
        }
    }
}

fn kmp(text: &Vec<bool>, pattern: &Vec<bool>, pi: &Vec<usize>) -> bool {
    let mut j = 0;
    for i in 0..2 * N {
        while j > 0 && text[i] != pattern[j] {
            j = pi[j - 1];
        }
        if text[i] == pattern[j] {
            if j == N - 1 {
                return true;
            } else {
                j += 1;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    ok!(())
}