// https://www.acmicpc.net/problem/11047
// O(N)

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v = input.split_ascii_whitespace().map(|s| s.parse::<usize>()).flatten();

    v.next();
    let mut k = v.next().unwrap();

    let mut cnt = 0;
    for coin in v.rev() {
        if k >= coin {
            cnt += k / coin;
            if k % coin == 0 {
                break
            } else {
                k %= coin;
            }
        }
    }

    println!("{cnt}");

    Ok(())
}