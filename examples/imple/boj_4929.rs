// https://www.acmicpc.net/problem/4929
// O(n + m)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = || io::stdin().lines().next().unwrap().unwrap();
    let mut ouput = BufWriter::new(io::stdout().lock());

    loop {
        let v1 = input();
        if v1 == "0" {
            break;
        }
        let v2 = input();

        let v1: Vec<i32> = v1.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
        let v2: Vec<i32> = v2.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

        let (mut ans, mut i, mut j) = (0, 0, 0);
        let (mut ns, mut ms) = (0, 0);

        while i < v1.len() && j < v2.len() {
            if v1[i] < v2[j] {
                i += 1;
            } else if v1[i] > v2[j] {
                j += 1;
            } else {
                let (sum1, sum2) = (v1[ns..i].iter().sum(), v2[ms..j].iter().sum());
                ans += i32::max(sum1, sum2) + v1[i];
                ns = i + 1;
                ms = j + 1;
                i += 1;
                j += 1;
            }
        }
        ans += i32::max(v1[ns..].iter().sum(), v2[ms..].iter().sum());

        writeln!(ouput, "{}", ans)?;
    }

    Ok(())
}