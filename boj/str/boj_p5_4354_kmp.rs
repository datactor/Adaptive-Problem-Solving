// https://www.acmicpc.net/problem/4354

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.trim().as_bytes()
        }
    }
}

macro_rules! ok {
    (()) => {
        {
            let mut read_buf = BufReader::new(io::stdin().lock());
            let mut write_buf = BufWriter::new(io::stdout().lock());
            let mut buf_to_string = String::new();

            while let billboard = read!(read_buf, buf_to_string) {
                if billboard[0] == b'.' { break }
                let len = billboard.len();
                let mut table: Vec<usize> = vec![0; len];
                let mut pi = 0;
                for (i, b) in billboard.iter().enumerate().skip(1) {
                    while pi > 0 && &billboard[pi] != b {
                        pi = table[pi - 1];
                    }
                    if &billboard[pi] == b {
                        pi += 1;
                        table[i] = pi;
                    }
                }

                let val = table.last().cloned().unwrap_or(0);

                writeln!(write_buf, "{}", if len % (len - val) != 0 { 1 } else { len / (len - val) })?;
            }

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    ok!(())
}