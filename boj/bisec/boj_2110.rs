// https://www.acmicpc.net/problem/2110

use std::{
    error::Error,
    io::{self, prelude::*},
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            input: s.split_ascii_whitespace(),
        }
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, c) = (scanner.read::<usize>(), scanner.read::<i32>());

    let mut nodes: Vec<i32> = (0..n).map(|_| scanner.read::<i32>()).collect();
    nodes.sort();

    let mut max = (nodes[n - 1] - nodes[0]) / (c - 1); // 가장 인접한 공유기 사이의 거리가 평균치보다 클 수 없다.
    let (mut left, mut right) = (1, max);

    // // distance를 가중 +O(N)
    // let mut distances: Vec<i32> = (1..n).map(|i| nodes[i]-nodes[i-1]).collect();
    //
    // while left <= right {
    //     let mid = (left + right) / 2;
    //     let mut cnt = 0;
    //     let mut dist = 0;
    //     for i in &distances {
    //         dist += i;
    //         if dist >= mid {
    //             cnt += 1;
    //             dist = 0;
    //         }
    //     }
    //     if cnt >= c-1 {
    //         left = mid + 1;
    //     } else {
    //         right = mid - 1;
    //     }
    // }

    while left <= right {
        let mid = (left + right) / 2;
        let mut cnt = 1;
        let mut dist = nodes[0]; // 0번째 포지션(가장 작은 포지션)을 초기 dist로 설정
        for i in nodes[1..n].iter() {
            if *i >= mid + dist {
                // node거리가 mid + dist보다 크거나 같을 경우에만 연산
                dist = *i; // dist를 갱신
                cnt += 1
            }
        }
        if cnt >= c {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", right);

    Ok(())
}
