// https://www.acmicpc.net/problem/2565
// O(N.pow(2))

use std::{
    io::{self, prelude::*},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v: Vec<(usize, usize)> = input.lines()
        .skip(1)
        .map(|s| {
            let mut i = s.split_ascii_whitespace().map(|y| y.parse::<usize>()).flatten();
            (i.next().unwrap_or_else(|| 0), i.next().unwrap_or_else(|| 0))
        })
        .collect();
    v.sort();

    let mut arr = vec![0; v.len()];

    for i in 0..v.len() {
        for j in 0..i {
            if v[i].1 > v[j].1 && arr[i] < arr[j] {
                arr[i] = arr[j]
            }
        } arr[i] += 1;
    }

    println!("{}", v.len() - arr.iter().max().unwrap_or(&0));
    Ok(())
}