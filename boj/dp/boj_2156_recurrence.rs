// // https://www.acmicpc.net/problem/2156

use std::{
    cmp::max,
    error::Error,
    io::{self, Read},
    str::FromStr,
};

fn max_total_value(v: &[usize]) -> usize {
    let mut arr = [0; 10_000];
    match v.len() {
        0 => 0,
        1 => v[0],
        2 => v[0] + v[1],
        _ => {
            arr[0] = v[0];
            arr[1] = v[0] + v[1];
            arr[2] = max(arr[1], max(v[0] + v[2], v[1] + v[2]));
            for i in 3..v.len() {
                arr[i] = max(
                    arr[i - 1],
                    max(arr[i - 2] + v[i], arr[i - 3] + v[i - 1] + v[i]),
                );
            }
            arr[v.len() - 1]
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let v: Vec<usize> = input
        .split_whitespace()
        .skip(1)
        .map(|s| usize::from_str(s).unwrap())
        .collect();

    println!("{}", max_total_value(&v));
    Ok(())
}
