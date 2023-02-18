// https://www.acmicpc.net/problem/9663

use std::io::{self, prelude::*};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let mut x = vec![0; n];
    let solve = nqueen(n, &mut x);
    println!("{}", solve);
}

fn nqueen(n: usize, x: &mut Vec<usize>) -> usize {
    if n == 0 {
        return 1;
    }
    let mut cnt = 0;
    for i in 0..x.len() {
        if x[i] != 0 {
            continue
        }

        let is_promise = x
            .iter()
            .enumerate()
            .filter(|(_, &v)| v != 0)
            .map(|(i, &v)| (i as i32 - v as i32, i + v))
            .any(|(a, b)| a == i as i32 - n as i32 || b == i + n);
        if !is_promise {
            x[i] = n;
            cnt += nqueen(n-1, x);
            x[i] = 0;
        }
    }
    cnt
}