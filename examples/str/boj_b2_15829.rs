// https://www.acmicpc.net/problem/15829

use std::{
    io::{self, Read, Write, BufWriter},
};

const M: u64 = 1_234_567_891;

fn main() -> io::Result<()> {
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let v = buffer.split_ascii_whitespace();
    let chars = v.skip(1).next().unwrap().as_bytes();
    let l = chars.len();

    let mut hash = 0;
    let mut r = 1;
    for i in 0..l {
        let a = (chars[i] - 96) as u64;
        hash = (hash + a * r) % M;
        r = (r * 31) % M;
    }
    writeln!(writer, "{}", hash)?;

    Ok(())
}