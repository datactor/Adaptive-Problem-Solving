// https://www.acmicpc.net/problem/25682
// O(N * M * K)


use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut first_line = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = first_line.next().unwrap();
    let m = first_line.next().unwrap();
    let k = first_line.next().unwrap();

    let board: Vec<_> = (0..n).map(|_|
        lines.next().unwrap().as_bytes()
    ).collect();

    let mut arr_b = vec![vec![0; m+1]; n+1];
    let mut arr_w = vec![vec![0; m+1]; n+1];

    let mut min_b = vec![vec![0; m+1]; n+1];
    let mut min_w = vec![vec![0; m+1]; n+1];

    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if (i+j-2)%2 == 0 {
                if board[i-1][j-1] == 87 {
                    arr_b[i-1][j-1] += 1
                } else {
                    arr_w[i-1][j-1] += 1
                }
            } else {
                if board[i-1][j-1] == 87 {
                    arr_w[i-1][j-1] += 1
                } else {
                    arr_b[i-1][j-1] += 1
                }
            }
            if j >= k {
                // 합산 dp
                if i-1 == 0 {
                    min_b[i-1][j-1] = arr_b[i-1][j-k..j].iter().sum::<i32>();
                    min_w[i-1][j-1] = arr_w[i-1][j-k..j].iter().sum::<i32>();
                } else {
                    min_b[i-1][j-1] = min_b[i-2][j-1] + arr_b[i-1][j-k..j].iter().sum::<i32>();
                    min_w[i-1][j-1] = min_w[i-2][j-1] + arr_w[i-1][j-k..j].iter().sum::<i32>();
                }

                // // transpose용
                // min_b[i-1][j-1] = arr_b[i-1][j-k..j].iter().sum::<i32>();
                // min_w[i-1][j-1] = arr_w[i-1][j-k..j].iter().sum::<i32>();
            }
        }
    }

    // println!("dp");
    let mut min = 2000_000;
    for i in k-1..n {
        for j in k-1..m {
            if i < k {
                // println!("{}, {}", i, min_w[i][j]);
                min = min.min(min_w[i][j]);
                min = min.min(min_b[i][j]);
            } else {
                min = min.min(min_w[i][j] - min_w[i-k][j]);
                min = min.min(min_b[i][j] - min_b[i-k][j]);
            }
        }
    }

    println!("{}", min);


    // println!("transpose");
    // for i in k-1..m {
    //     for j in k-1..n {
    //         println!("{}", min_w[j][i]);
    //     }
    // }
    // println!("////////////////////");
    // let min_w = transpose(min_w);
    // let min_b = transpose(min_b);
    // for i in k-1..m {
    //     println!("{:?}", min_w[i]);
    // }
    // println!("////////////////////");
    // for i in k-1..m {
    //     println!("{:?}", min_b[i]);
    // }
    // let mut min = 2000_000;
    // for i in k-1..m {
    //     for j in 0..n-k+1 {
    //         // println!("{}", j);
    //         // println!("{}", min_b[i][j..j+k-1].iter().sum::<i32>());
    //         min = min.min(min_b[i][j..j+k].iter().sum::<i32>());
    //         min = min.min(min_w[i][j..j+k].iter().sum::<i32>())
    //     }
    //     // println!("{:?}", min_b[i]);
    // }
    // println!("{}", min);
    // println!("{:?}", min_b);



    // // 시간초과!!
    // for i in 0..m-k+1 {
    //     // println!("{}", tmp_b);
    //     for j in 0..n-k+1 {
    //         let mut tmp_b = 0;
    //         let mut tmp_w = 0;
    //         for x in j..j+k {
    //             tmp_b += arr_b[x][i..i+k].iter().sum::<i32>();
    //             // println!("arr_b[{}][{}..{}].iter().sum() = {}", x, i, i+k, tmp_b);
    //             tmp_w += arr_w[x][i..i+k].iter().sum::<i32>();
    //             // println!("{}", x);
    //             // println!("x = {}, {}, {}", x, i, i+k);
    //         }
    //         // println!("{}", tmp_b);
    //         // println!("{}", tmp_w);
    //         min = min.min(tmp_b);
    //         min = min.min(tmp_w);
    //     }
    // }
    // println!("{}", min);

    Ok(())
}

// 행렬 바꿔서 풀기. 풀리지만 매우 느림
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}