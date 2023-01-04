// https://www.acmicpc.net/problem/2565
// O(N.pow(2))

use std::{
    io::{self, prelude::*},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v: Vec<(usize, usize)> = input.lines()
        .skip(1)
        .map(|s| {
            let mut i = s.split_ascii_whitespace().map(|y| y.parse::<usize>()).flatten();
            (i.next().unwrap_or_else(|| 0), i.next().unwrap_or_else(|| 0))
        })
        .collect();
    v.sort();

    let mut arr = vec![0; v.len()];

    for i in 0..v.len() {
        for j in 0..i {
            if v[i].1 > v[j].1 && arr[i] < arr[j] { // v[i]가 더 크면 +1을 할 근거(IS)
                arr[i] = arr[j]                     // 그 배열의 수가 최대일 경우(arr[i] < arr[j]; LIS)
            }                                       // arr에 그 값을 넣고 +1을 해서 저장함
        } arr[i] += 1;
    }
    Ok(())
}