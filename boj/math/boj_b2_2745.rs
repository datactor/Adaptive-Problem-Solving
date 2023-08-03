// https://www.acmicpc.net/problem/2745

use std::{
    error::Error,
    io::{self, BufRead, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;

    let mut iter = buffer.split_ascii_whitespace();
    let b = iter.next().ok_or("parse fail")?.as_bytes();
    let n = iter.next().ok_or("parse fail")?.parse::<usize>()?;

    let mut x = 1;
    let mut ans = 0;
    for i in (0..b.len()).rev() {
        let c = b[i];
        if c >= b'A' as u8 {
            ans += (c - b'A' + 10) as usize * x;
        } else {
            ans += (c - b'0') as usize * x;
        }
        x *= n;
    }
    write!(buf_writer, "{}", ans)?;
    Ok(())
}
