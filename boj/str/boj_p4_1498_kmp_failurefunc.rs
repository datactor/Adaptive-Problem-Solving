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

            // Init failure func idx for KMP
            let mut failure_idx = vec![0; txt.len()];

            // KMP
            // Compute the failure func idx using KMP
            // For each position in the txt,
            // this loop calculates the length of the longest proper prefix which is also a suffix.
            for target_txt_len in 1..txt.len() {
                let mut common_prefix_sufix_len = failure_idx[target_txt_len-1];
                while common_prefix_sufix_len > 0 && txt[target_txt_len] != txt[common_prefix_sufix_len] {
                    common_prefix_sufix_len = failure_idx[common_prefix_sufix_len-1];
                }
                if txt[target_txt_len] == txt[common_prefix_sufix_len] {
                    failure_idx[target_txt_len] = common_prefix_sufix_len + 1;
                }
            }

            // Iterate through the txt and check if any substring can be
            // represented as a pattern repeated multiple times
            for target_txt_len in 1..txt.len() {
                let prefix_len = target_txt_len + 1 - failure_idx[target_txt_len];

                // If the current substring length is divisible by the prefix length,
                // it means that the substring can be represented by repeating the prefix.
                // If prefix_len is divisible by target_txt_len,
                // it means it is repeated and becomes a pattern.
                if failure_idx[target_txt_len] > 0 && (target_txt_len+1) % prefix_len == 0 {
                    writeln!(write_buf, "{} {}", target_txt_len + 1, (target_txt_len+1) / prefix_len)?;
                }
            }

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    ok!(())
}