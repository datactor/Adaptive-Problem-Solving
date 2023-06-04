use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    cmp::max,
};

macro_rules! read_num {
    ($reader:expr, $type:ty) => {
        {
            $reader
                .next()
                .expect("Failed to get next line")
                .expect("Failed to read line")
                .trim()
                .parse::<$type>()
                .expect("Failed to parse")
        }
    }
}

macro_rules! find_num {
    ($reader:expr, $n:expr, $type:ty) => {
        {
            let mut vec = vec![0; $n];
            let mut dp = vec![0; $n];
            let mut mx = 0;
            let mut line = $reader
                                .next()
                                .expect("Failed to get next line")
                                .expect("Failed to read line");
            let mut iter = line.split_ascii_whitespace();
            for i in 0..$n {
                let next = iter.next().expect("Failed to get next iter").parse::<$type>().expect("Failed to parse");
                vec[i] = next;
                if i == 0 {
                    dp[i] = next;
                    mx = next;
                } else {
                    dp[i] = max(next, next + dp[i-1]);
                    mx = max(mx, dp[i]);
                }
            }
            mx
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf_to_lines = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let n = read_num!(read_buf_to_lines, usize);
    writeln!(write_buf, "{}", find_num!(read_buf_to_lines, n, i32))?;

    Ok(())
}