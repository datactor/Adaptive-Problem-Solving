// https://www.acmicpc.net/problem/9251
// O(M * N)

use std::{
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let v: Vec<_> = input
        .split_ascii_whitespace()
        .map(|s| s.as_bytes())
        .collect();

    let mut arr = vec![0; v[1].len()];

    for i in 0..v[0].len() {
        let mut cnt = 0;
        for j in 0..v[1].len() {
            if cnt < arr[j] {
                // cnt가 arr[j] 'j'번째 arr element보다 작을 경우
                cnt = arr[j] // 일단 longest 배열의 길이를 cnt에 올려놓는다.
            } else if v[0][i] == v[1][j] {
                // v[0][i]와 v[1][j]가 같을 경우
                arr[j] = cnt + 1 // j번째 arr element에 cnt + 1을 올린다. 이게 왜 되냐면, v[0][i]와의 비교가 끝난
            } // longest 배열의 길이를 'j'번째 arr element에 올려놨었고 그것을 cnt로 가져왔기 때문에 가능함.
        } // 즉 arr[j]는 첫번째 for문 안의 i의 lcs를 골라서 최대 배열의 길이를 올려놓고, 같을 때만 기록하면 됨.
    }

    println!("{}", arr.iter().max().unwrap());
    Ok(())
}
