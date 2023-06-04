// https://www.acmicpc.net/problem/13305
// O(N)

use std::{error::Error, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>()?;

    let dsts: Vec<usize> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let prices: Vec<usize> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut tmp = 0;
    let mut sum = 0;
    for i in 0..n - 1 {
        sum += prices[tmp] * dsts[i];
        if prices[tmp] > prices[i + 1] {
            tmp = i + 1
        }
    }
    println!("{}", sum);

    Ok(())
}
