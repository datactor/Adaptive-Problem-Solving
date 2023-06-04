// https://www.acmicpc.net/problem/11725

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_line_to_num {
    ($reader:expr, $input:expr, $type: ty) => {
        {
            $reader.read_line(&mut $input)?;
            $input.trim().parse::<$type>()
        }
    }
}

macro_rules! read_to_vec {
    ($reader:expr, $input:expr, $n:expr, $type: ty) => {
        {
            let mut vec = vec![vec![]; $n+1];
            for _ in 0..$n-1 {
                $input.clear();
                $reader.read_line(&mut $input).expect("Failed to read");
                let mut iter = $input.split_ascii_whitespace();
                let n = iter.next().expect("Failed to 1st iter").parse::<$type>().expect("Failed to parse 1st");
                let m = iter.next().expect("Failed to 2nd iter").parse::<$type>().expect("Failed to parse 2nd");
                vec[n as usize].push(m);
                vec[m as usize].push(n);
            }
            vec
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let Ok(n) = read_line_to_num!(read_buf, buf_to_string, usize) {
        let vec = read_to_vec!(read_buf, buf_to_string, n, usize);

        let mut dq = std::collections::VecDeque::from([1]);
        let mut parents = vec![0; n+1];

        while let Some(parent) = dq.pop_front() {
            for child in &vec[parent] {
                if parents[*child] == 0 && *child != 1 {
                    parents[*child] = parent;
                    dq.push_back(*child);
                }
            }
        }
        for i in 2..=n {
            writeln!(write_buf, "{}", parents[i])?;
        }
    }

    Ok(())
}