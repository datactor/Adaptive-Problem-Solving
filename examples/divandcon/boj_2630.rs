// https://www.acmicpc.net/problem/2630

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main () -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let v: Vec<Vec<usize>> = (0..n).map(|_| lines.next().unwrap().split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect()).collect();

    let (w, b) = dc(0, 0, n, &v);
    print!("{}\n{}", w, b);
    Ok(())
}

fn dc(x: usize, y: usize, n: usize, v: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut tmp = 0;
    for i in x..x+n {
        for j in y..y+n {
            if v[i][j] == 1 {
                tmp += 1
            }
        }
    }

    if tmp == 0 {
        return (1, 0)
    } else if tmp == n.pow(2) {
        return (0, 1)
    } else {
        let (w1, b1) = dc(x, y, n/2, v);
        let (w2, b2) = dc(x+n/2, y, n/2, v);
        let (w3, b3) = dc(x, y+n/2, n/2, v);
        let (w4, b4) = dc(x+n/2, y+n/2, n/2, v);
        return (w1 + w2 + w3 + w4, b1 + b2 + b3 + b4)
    }
}