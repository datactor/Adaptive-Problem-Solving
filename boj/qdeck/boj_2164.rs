// https://www.acmicpc.net/problem/2164
// O(N)

use std::{
    collections::VecDeque,
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n = input.trim().parse::<usize>().unwrap();
    let mut deck: VecDeque<usize> = (1..n + 1).map(|s| s).collect();
    while deck.len() > 1 {
        deck.pop_front();
        deck.rotate_left(1);
    }

    println!("{}", deck[0]);

    Ok(())
}
