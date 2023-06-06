// https://www.acmicpc.net/problem/10610

use std::{
    io::{self, BufRead},
    fmt::Write,
};

macro_rules! solve {
    () => {
        {
            let mut buffer = String::new();
            io::stdin().lock().read_line(&mut buffer).unwrap();
            let mut sum = 0;
            let mut bytes = buffer.trim().as_bytes().to_vec();
            bytes.sort_by(|a, b| b.cmp(a));
            let len = bytes.len();

            buffer.clear();
            for b in &bytes {
                let n = (b - b'0') as u32;
                sum += n;
                write!(buffer, "{}", n).unwrap();
            }

            if bytes[len-1] != b'0' || sum % 3 != 0 {
                Err(io::Error::new(io::ErrorKind::Other, "-1"))
            } else {
                Ok(buffer)
            }
        }
    }
}

fn main() {
    match solve!() {
        Ok(buffer) => {
            print!("{}", buffer);
        },
        Err(_) => {
            print!("-1");
        },
    }
}