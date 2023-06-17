// https://www.acmicpc.net/problem/1498

use std::{
    io::{self, Write, BufWriter},
};

macro_rules! ok {
    (()) => {
        {
            let mut write_buf = BufWriter::new(io::stdout().lock());
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let txt = buffer.trim().as_bytes();

            // 시간초과
            // for i in 1..txt.len() {
            //     let mut table = vec![0; i+1];
            //     let mut pi = 0;
            //     let pattern = &txt[0..i+1];
            //     for (j, b) in pattern.iter().enumerate().skip(1) {
            //         while pi > 0 && &pattern[pi] != b {
            //             pi = table[pi-1];
            //         }
            //         if &pattern[pi] == b {
            //             pi += 1;
            //             table[j] = pi;
            //         }
            //     }
            //
            //     let val = table.last().cloned().unwrap_or(0);
            //
            //     let x = (i+1) % (i+1-val);
            //     if x == 0 && (i+1) / (i+1-val) != 1 {
            //         writeln!(write_buf, "{} {}", i+1, (i+1) / (i+1-val))?;
            //     }
            // }

            let mut failure_idx = vec![0; txt.len()];

            for i in 1..txt.len() {
                let mut j = failure_idx[i-1];
                while j > 0 && txt[i] != txt[j] {
                    j = failure_idx[j-1];
                }
                if txt[i] == txt[j] {
                    failure_idx[i] = j + 1;
                }
            }

            for i in 0..txt.len() {
                let prefix_len = i + 1 - failure_idx[i];
                if failure_idx[i] > 0 && (i+1) % prefix_len == 0 {
                    writeln!(write_buf, "{} {}", i + 1, (i+1) / prefix_len)?;
                }
            }

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    ok!(())
}