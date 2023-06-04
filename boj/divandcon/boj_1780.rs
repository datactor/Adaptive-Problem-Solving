// https://www.acmicpc.net/problem/1780

use std::{
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<i32>().unwrap();

    let v: Vec<Vec<i32>> = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let (a, b, c) = dc(0, 0, n, &v);
    print!("{}\n{}\n{}", a, b, c);
    Ok(())
}

fn dc(x: i32, y: i32, n: i32, v: &Vec<Vec<i32>>) -> (i32, i32, i32) {
    let mut tmp_m1 = 0;
    let mut tmp_0 = 0;
    let mut tmp_1 = 0;
    for i in x..x + n {
        for j in y..y + n {
            match v[i as usize][j as usize] {
                0 => tmp_0 += 1,
                1 => tmp_1 += 1,
                _ => tmp_m1 += 1,
            }
        }
    }

    if tmp_m1 == n.pow(2) {
        return (1, 0, 0);
    } else if tmp_0 == n.pow(2) {
        return (0, 1, 0);
    } else if tmp_1 == n.pow(2) {
        return (0, 0, 1);
    } else {
        let (a1, b1, c1) = dc(x, y, n / 3, v);
        let (a2, b2, c2) = dc(x + n / 3, y, n / 3, v);
        let (a3, b3, c3) = dc(x + 2 * n / 3, y, n / 3, v);
        let (a4, b4, c4) = dc(x, y + n / 3, n / 3, v);
        let (a5, b5, c5) = dc(x + n / 3, y + n / 3, n / 3, v);
        let (a6, b6, c6) = dc(x + 2 * n / 3, y + n / 3, n / 3, v);
        let (a7, b7, c7) = dc(x, y + 2 * n / 3, n / 3, v);
        let (a8, b8, c8) = dc(x + n / 3, y + 2 * n / 3, n / 3, v);
        let (a9, b9, c9) = dc(x + 2 * n / 3, y + 2 * n / 3, n / 3, v);
        return (
            a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9,
            b1 + b2 + b3 + b4 + b5 + b6 + b7 + b8 + b9,
            c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8 + c9,
        );
    }
}
