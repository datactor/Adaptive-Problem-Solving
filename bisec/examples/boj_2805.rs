// https://www.acmicpc.net/problem/2805

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace().skip(1).map(|s| s.parse::<i32>().unwrap());
    let m = input.next().unwrap();
    let v: Vec<i32> = input.collect();
    
    let (mut left, mut right) = (0, 999_999_999);

    while left <= right {
        let mut sum = 0;
        let mut mid = (left + right) / 2;
        for i in &v {
            if i - mid > 0 {
                sum += i - mid;
            }
            if sum > 2_000_000_000 {
                break
            }
        }
        if sum >= m {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", right);

    Ok(())
}