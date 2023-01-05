// https://www.acmicpc.net/problem/12865
// O(N * K)

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let mut first_line = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();
    let n = first_line.next().unwrap();
    let k = first_line.next().unwrap();

    let mut v: Vec<(usize, usize)> = lines.map(
        |s| {
            let mut a = s.split_ascii_whitespace()
                .map(|y| y.parse::<usize>()).flatten();
            (a.next().unwrap(), a.next().unwrap())
        }
    ).collect();

    v.sort();

    let mut arr = vec![0; k+1];

    // Knapsack algorithm
    for &(w, v) in v.iter() {
        for j in (1..k+1).rev() {
            if j >= w {
                arr[j] = arr[j].max(arr[j-w] + v);
            } else {
                break
            }
        }
    }

    println!("{}", arr[k]);
    Ok(())
}