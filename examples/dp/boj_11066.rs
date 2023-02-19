// https://www.acmicpc.net/problem/11066

use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     let mut output = BufWriter::new(io::stdout().lock());
//     io::stdin().read_to_string(&mut input)?;
//
//     let mut sc = Scanner::new(&input);
//     let t = sc.read::<usize>();
//     for _ in 0..t {
//         let k = sc.read::<usize>();
//         let chapters: Vec<usize> = (0..k).map(|_| sc.read::<usize>()).collect();
//         let mut dp = vec![vec![0; k]; k];
//
//         // dp[i][j] = i에서 j까지의 비용
//         // dp[i][j]의 최소비용은 dp[i][j-1] + chapters[j] + min으로 나타낼 수 있다.
//
//         // 최소값을 결정하는 것은 마지막 비용(마지막 계산은 순서와 상관없이 같다)이 아닌 이전의 비용이 결정한다.
//
//         // 여기서 dp[i][j-1] + chapters[j]를 계산해준다.
//         for i in 0..k-1 {
//             dp[i][i+1] = chapters[i] + chapters[i+1];
//             for j in i+2..k {
//                 dp[i][j] = dp[i][j-1] + chapters[j];
//             }
//         }
//         println!("{:?}", dp);
//
//         for i in 2..k {
//             for j in 0..k-i {
//                 // dp[j][j..j+i] + dp[j+1..j+1+i][j+i]의 최소값을 구해준다.
//                 dp[j][j+i] += (j..j+i).map(|s| dp[j][s] + dp[s+1][j+i]).min().unwrap()
//             }
//         }
//         writeln!(output, "{}", dp[0][k-1])?;
//     }
//
//     Ok(())
// }

// flatten
fn get_index(i: usize, j: usize, k: usize) -> usize {
    i * k + j
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let t = sc.read::<usize>();
    for _ in 0..t {
        let k = sc.read::<usize>();
        let v: Vec<usize> = (0..k).map(|_| sc.read::<usize>()).collect();
        let mut dp = vec![0; k * k];

        for i in 0..k - 1 {
            dp[get_index(i, i + 1, k)] = v[i] + v[i + 1];
            for j in i + 2..k {
                dp[get_index(i, j, k)] = dp[get_index(i, j - 1, k)] + v[j];
            }
        }

        for i in 2..k {
            for j in 0..k - i {
                let mut min_sum = std::usize::MAX;
                for s in j..j + i {
                    let left_sum = dp[get_index(j, s, k)];
                    let right_sum = dp[get_index(s + 1, j + i, k)];
                    min_sum = std::cmp::min(min_sum, left_sum + right_sum);
                }
                dp[get_index(j, j + i, k)] += min_sum;
            }
        }
        writeln!(output, "{}", dp[get_index(0, k - 1, k)])?;
    }

    Ok(())
}
