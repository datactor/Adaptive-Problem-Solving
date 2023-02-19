// https://www.acmicpc.net/problem/12865
// O(N * K)

use std::{
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let mut first_line = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();
    let n = first_line.next().unwrap();
    let k = first_line.next().unwrap();

    let mut v: Vec<(usize, usize)> = lines
        .map(|s| {
            let mut a = s
                .split_ascii_whitespace()
                .map(|y| y.parse::<usize>())
                .flatten();
            (a.next().unwrap(), a.next().unwrap())
        })
        .collect();

    v.sort();

    let mut arr = vec![0; k + 1];

    // Knapsack algorithm
    for &(w, v) in v.iter() {
        for j in (1..k + 1).rev() {
            if j >= w {
                // j는 load(가방의 적재 가능무게), w는 물건의 무게. 물건의 무게보다 적재가능량이 크면
                arr[j] = arr[j].max(arr[j - w] + v); // j번째 arr에 arr[j]의 [기존 값],
                                                     // [j-w(w가 들어갈 수 있는 가방)에 들어있는 아이템들의 가치 + 새로 넣을 아이템의 가치(v)를 더한 값]
                                                     // 중 큰 값을 arr[j]에 저장
            } else {
                break; // 가방이 버티질 못하는 무게에선 새로운 아이템을 고려함
            }
        }
    }

    println!("{}", arr[k]);
    Ok(())
}
