// https://www.acmicpc.net/problem/2579

use std::{
    error::Error,
    io::{self, prelude::*, BufReader, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let v: Vec<usize> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let result = if v.len() == 1 {
        v[0]
    } else if v.len() == 2 {
        v[0] + v[1]
    } else {
        let mut arr = vec![0; v.len()];
        arr[0] = v[0];
        arr[1] = v[0] + v[1];
        arr[2] = (arr[0] + v[2]).max(v[1] + v[2]);
        for i in 3..v.len() {
            arr[i] = (arr[i - 2] + v[i]).max(arr[i - 3] + v[i - 1] + v[i])
        }
        arr[v.len() - 1]
    };

    writeln!(output, "{}", result)?;
    Ok(())
}
