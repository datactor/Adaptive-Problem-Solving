// https://www.acmicpc.net/problem/9252
// O(N * M)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

// // 풀리긴 하지만 memoization을 효율적으로 사용했다고 할 수는 없다.
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     let mut output = BufWriter::new(io::stdout().lock());
//     io::stdin().read_to_string(&mut input)?;
//
//     let strings: Vec<_> = input.split_ascii_whitespace().map(|s| s.as_bytes()).collect();
//
//     let mut lens = vec![0; b.len()];
//     let mut vals = vec![vec![]; b.len()];
//
//     for i in 0..a.len() {
//         let mut cnt_and_val = (0, vec![]);
//         for j in 0..b.len() {
//             if cnt_and_val.0 < lens[j] {
//                 cnt_and_val = (lens[j], vals[j].clone())
//             } else if a[i] == b[j] {
//                 let mut tmp = cnt_and_val.clone();
//                 lens[j] = tmp.0 + 1;
//                 tmp.1.push(b[j]);
//                 vals[j] = tmp.1;
//             }
//         }
//     }
//
//     let max = lens.iter().enumerate().map(|(i, v)| (v, i)).max().unwrap();
//
//     writeln!(output, "{}", max.0)?;
//     writeln!(output, "{}", std::str::from_utf8(&vals[max.1]).unwrap())?;
//
//     Ok(())
// }


struct Scanner<'a> {
    sample_a: &'a [u8],
    sample_b: &'a [u8],
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner {
        let mut tmp = s.split_ascii_whitespace().map(|s| s.as_bytes());
        Scanner {
            sample_a: tmp.next().unwrap(),
            sample_b: tmp.next().unwrap(),
        }
    }

    fn find_lcs(&self) -> (i32, Vec<u8>) {
        let mut table = vec![vec![0; self.sample_a.len()+1]; self.sample_b.len()+1];
        let mut max = 0;

        for (i, val) in self.sample_b.iter().enumerate() {
            for j in 0..self.sample_a.len() {
                table[i+1][j+1] =
                    if val == &self.sample_a[j] {
                        table[i][j]+1
                    } else {
                        table[i][j+1].max(table[i+1][j])
                    };

                max = table[i+1][j+1].max(max);
            }
        }

        let mut lcs = Vec::new();
        let (mut i, mut j) = (self.sample_b.len(), self.sample_a.len());

        while table[i][j] > 0 {
            if table[i][j] == table[i-1][j] {
                i -= 1;
                continue;
            }
            if table[i][j] == table[i][j-1] {
                j -= 1;
                continue;
            }

            lcs.push(self.sample_a[j-1]);
            i -= 1;
            j -= 1;
        }
        return (max, lcs)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let (max, mut lcs) = sc.find_lcs();

    writeln!(output, "{}", max)?;
    if !lcs.is_empty() {
        lcs.reverse();
        writeln!(output, "{}", std::str::from_utf8(&lcs).unwrap())?;
    }

    Ok(())
}