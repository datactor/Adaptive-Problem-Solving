use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();
    let mut read = || input.next().unwrap();

    let (n, k) = (read(), read());
    let mut sum = 0;
    let v: Vec<usize> = (0..n).map(|_| {
        let x = read();
        sum += x;
        x
    }).collect();

    let mut mid = sum / k;

    let (mut left, mut right) = (1, mid);
    while left <= right  {
        let mut cnt = 0;
        for i in &v {
            cnt += i / mid;
        }
        if cnt >= k {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
        mid = (left + right) / 2;
    }

    println!("{}", mid);

    Ok(())
}