// https://www.acmicpc.net/problem/16139
// O(N * 26)

use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let s = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();

    let mut arr = vec![[0; 26]; s[0].len()];
    arr[0][s[0][0] as usize - 97] = 1;

    for i in 1..s[0].len() {
        let j = (s[0][i] - 97) as usize;
        arr[i] = arr[i - 1];
        arr[i][j] = arr[i - 1][j] + 1;
    }

    for line in lines.skip(1) {
        let mut v = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<String>())
            .flatten();

        let a = (v.next().unwrap().as_bytes()[0] - 97) as usize;
        let l = v.next().unwrap().parse::<usize>().unwrap();
        let r = v.next().unwrap().parse::<usize>().unwrap();

        writeln!(
            output,
            "{}",
            match l {
                0 => arr[r][a],
                _ => arr[r][a] - arr[l - 1][a],
            }
        )?;
    }

    Ok(())
}
