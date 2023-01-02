// https://www.acmicpc.net/problem/11054
// O(N.pow(2))

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let v: Vec<usize> = input.split_ascii_whitespace().skip(1).map(
        |s| s.parse::<usize>().unwrap()).collect();

    let mut reversed_v = v.clone();
    reversed_v.reverse();

    let mut ascend = vec![1; v.len()];
    let mut descend = vec![1; v.len()];

    for i in 0..v.len() {
        for j in 0..i {
            if v[i] > v[j] {
                ascend[i] = ascend[i].max(ascend[j] + 1);
            }
            if reversed_v[i] > reversed_v[j] {
                descend[i] = descend[i].max(descend[j] + 1);
            }
        }
    }
    let sums: Vec<usize> = (0..v.len())
        .map(|i| ascend[i] + descend[v.len() - i - 1] - 1)
        .collect();
    println!("{}", sums.iter().max().unwrap());
    Ok(())
}