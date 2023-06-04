// https://www.acmicpc.net/problem/1931
// O(N lgN)

use std::{
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut conf: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let mut se = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>())
                .flatten();
            (se.next().unwrap(), se.next().unwrap())
        })
        .collect();

    conf.sort();
    conf.sort_by_key(|k| k.1);

    let mut last = conf[0].1;
    let mut cnt = 1;

    for i in 0..n - 1 {
        if last <= conf[i + 1].0 {
            cnt += 1;
            last = conf[i + 1].1;
        }
    }

    println!("{cnt}");

    Ok(())
}
