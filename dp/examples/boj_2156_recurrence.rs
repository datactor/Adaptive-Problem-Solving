// https://www.acmicpc.net/problem/2156

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v = input
        .split_ascii_whitespace()
        .skip(1)
        .map(
        |s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let max = match v.len() {
        1 => v[0],
        2 => v[0] + v[1],
        _ => {
            let mut arr = vec![0; v.len()];
            arr[0] = v[0];
            arr[1] = v[0] + v[1];
            arr[2] = [
                arr[1],
                v[0] + v[2],
                v[1] + v[2]
            ].into_iter().max().unwrap();
            for i in 3..v.len() {
                arr[i] = [
                    arr[i - 1],
                    arr[i - 2] + v[i],
                    arr[i - 3] + v[i - 1] + v[i],
                ].into_iter().max().unwrap();
            }
            arr[v.len()-1]
        }
    };

    writeln!(output, "{}", max)?;
    Ok(())
}