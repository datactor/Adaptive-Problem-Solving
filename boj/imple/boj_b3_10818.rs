// https://www.acmicpc.net/problem/10818

use std::{
    io::{self, Write, Read, BufWriter},
};

macro_rules! find {
    () => {
        {
            let mut input = String::new();
            io::stdin().lock().read_to_string(&mut input)?;
            let iter = input.split_ascii_whitespace().skip(1).map(|s| s.parse::<i32>()).flatten();
            let mut mn = 1_000_000;
            let mut mx = -1_000_000;

            for num in iter {
                if num < mn { mn = num }
                if num > mx { mx = num }
            }

            write!(BufWriter::new(io::stdout().lock()), "{} {}", mn, mx)?
        }
    }
}

fn main() -> io::Result<()> {
    find!();
    Ok(())
}
