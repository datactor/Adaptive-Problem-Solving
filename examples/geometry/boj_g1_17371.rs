// https://www.acmicpc.net/problem/17371

use std::io::{self, BufRead};

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2)
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(|line| line.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut conv = vec![];

    for _ in 0..n {
        let line = lines.next().unwrap();
        let coords: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        conv.push((coords[0], coords[1]));
    }

    let mut min = i32::MAX;
    let mut min_idx = -1;

    for i in 0..n {
        let mut max = -1;
        let mut max_idx = -1;
        for j in 0..n {
            let d = dist(conv[i], conv[j]);
            if max < d {
                max = d;
                max_idx = i as i32;
            }
        }
        if max < min {
            min = max;
            min_idx = max_idx;
        }
    }

    println!("{} {}", conv[min_idx as usize].0, conv[min_idx as usize].1);
}