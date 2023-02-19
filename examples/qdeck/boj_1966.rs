// https://www.acmicpc.net/problem/1966

use std::{
    collections::VecDeque,
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut f_line = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap());
        f_line.next().unwrap();
        let m = f_line.next().unwrap();

        let mut v: Vec<(usize, usize)> = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .enumerate()
            .map(|(idx, s)| (s.parse::<usize>().unwrap(), idx))
            .collect();

        let mut x = VecDeque::from(v.clone());
        v.sort();
        v.reverse();

        let mut cnt = 0;
        while v.len() > 0 {
            let a = x.pop_front().unwrap();
            if a.0 == v[0].0 {
                cnt += 1;
                v.remove(0);
                if a.1 == m {
                    break;
                }
            } else {
                x.push_back(a);
            }
        }
        writeln!(output, "{cnt}")?
    }
    Ok(())
}
