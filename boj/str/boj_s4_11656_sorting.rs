// https://www.acmicpc.net/problem/11656

use std::{
    io::{self, Write, BufRead, BufWriter},
};

macro_rules! ok {
    (()) => {
        {
            let mut input = String::new();
            io::stdin().lock().read_line(&mut input)?;

            let mut write_buf = BufWriter::new(io::stdout().lock());

            let mut v = Vec::new();
            for i in 0..input.trim().len() {
                let tmp = input[i..].to_string();
                v.push(tmp);
            }

            v.sort_by(|a, b| a.cmp(b));

            for element in v {
                write!(write_buf, "{}", element)?;
            }

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    ok!(())
}