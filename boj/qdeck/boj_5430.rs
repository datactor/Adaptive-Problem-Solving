#![allow(non_snake_case)]
use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
};

macro_rules! parse_line { ($($t: ty),+) => ({
  let mut line = String::new();
  io::stdin().read_line(&mut line).unwrap();
  let mut iter = line.split_whitespace();
  ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
})}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(io::stdout().lock());
    let t = parse_line!(usize);
    'case: for _ in 0..t {
        let cmd = parse_line!(String);
        let n = parse_line!(usize);
        let arr = parse_line!(String);

        let mut s = 0;
        let mut e = n;
        let mut rev = false;
        for c in cmd.chars() {
            if c == 'R' {
                rev = !rev;
                continue;
            }
            if e == s {
                writeln!(output, "error")?;
                continue 'case;
            }
            if rev {
                e -= 1
            } else {
                s += 1
            }
        }

        let mut arr: Vec<_> = arr[1..arr.len() - 1]
            .split(",")
            .skip(s)
            .take(e - s)
            .collect();
        if rev {
            arr.reverse()
        }
        writeln!(output, "[{}]", arr.join(","))?;
    }
    Ok(())
}
