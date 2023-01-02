// https://www.acmicpc.net/problem/11053
// O(N.pow(2))

use std::{
    io::{self, prelude::*},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v = input.split_ascii_whitespace().skip(1).map(
        |s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut arr = vec![1; v.len()];

    for i in 0..v.len() {
        for j in 0..i {
            let mut tmp = 0;
            if v[i] > v[j] {
                arr[i] = arr[i].max(arr[j] + 1)
            }
        }
    }
    println!("{}", arr.iter().max().unwrap());
    Ok(())
}