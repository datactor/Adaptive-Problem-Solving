// https://www.acmicpc.net/problem/1043

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut map = input.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let (n, m, k) = (map.next().unwrap(), map.next().unwrap(), map.next().unwrap());
    let mut t: usize = 0;
    for _ in 0..k {
        t += 1 << map.next().unwrap();
    }

    let mut v = Vec::<usize>::with_capacity(m);
    for _ in 0..m {
        let a = map.next().unwrap();
        let mut b = 0;

        for _ in 0..a {
            b += 1 << map.next().unwrap();
        }
        v.push(b);
    }

    let mut stack = Vec::<usize>::new();
    let mut visited = vec![false; n + 1];
    for i in 1..=n {
        if t & 1 << i == 1 << i {
            stack.push(i);
        }
    }

    while !stack.is_empty() {
        let a = stack.pop().unwrap();
        visited[a] = true;

        for grp in &v {
            if *grp & 1 << a == 1 << a {
                t |= *grp;
            }
        }

        for i in 1..=n {
            if !visited[i] && t & 1 << i == 1 << i {
                stack.push(i);
            }
        }
    }

    let mut cnt = 0;
    for i in &v {
        if t &*i == 0 {
            cnt += 1;
        }
    }

    print!("{cnt}");

    Ok(())
}
