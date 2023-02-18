// https://www.acmicpc.net/problem/1021

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v = input.split_ascii_whitespace().map(
        |s| s.parse::<i32>().unwrap());

    let n = v.next().unwrap();
    v.next().unwrap();
    let mut nums: Vec<i32> = v.collect();

    let mut deck: Vec<i32> = (1..n+1).map(|s| s).collect();

    let mut cnt = 0;
    while !nums.is_empty() {
        if deck[0] == nums[0] {
            deck.remove(0);
            nums.remove(0);
        } else {
            let idx = deck.iter().position(|&x| x == nums[0]).unwrap();
            let len = deck.len();
            if len - idx < idx {
                cnt +=  len - idx;
                deck.rotate_right( len - idx);
            } else {
                cnt += idx;
                deck.rotate_left(idx);
            }
        }
    }
    println!("{cnt}");

    Ok(())
}