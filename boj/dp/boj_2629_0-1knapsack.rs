// https://www.acmicpc.net/problem/2629
// O(n * max_weight)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
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

/// This is the standard 0/1 knapsack problem,
/// where each item can be either included or excluded from the knapsack.
///
/// The algorithm(knapsack) works as follows:
///
/// 1. Create an n x (W+1) array dp, where dp[i][j] represents the maximum value that can be obtained
/// by using a subset of the first i items with a maximum capacity of j.
///
/// 2. Initialize dp such that dp[0][j] = 0 for 0 <= j <= W, since there is no item to select from.
///
/// 3. For each item i from 1 to n, iterate through each capacity j from 0 to W.
///
/// 4. If w_i > j, set dp[i][j] = dp[i-1][j], since item i cannot be included
/// in the knapsack with capacity j.
///
/// 5. Otherwise, set dp[i][j] = max(dp[i-1][j], dp[i-1][j-w_i] + v_i), where max is the maximum function.
/// This means that we compare the maximum value that can be obtained without including item i (which is dp[i-1][j]) and
/// the maximum value that can be obtained by including item i (which is dp[i-1][j-w_i] + v_i).
///
/// 6. The maximum value that can be obtained by selecting a subset of the items with a total weight not exceeding W is given by dp[n][W].
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let n = sc.read::<usize>();
    let v: Vec<i32> = (0..n).map(|_| sc.read::<i32>()).collect();

    let m = sc.read::<usize>();
    let beads: Vec<i32> = (0..m).map(|_| sc.read::<i32>()).collect();

    let max_weight = v.iter().sum::<i32>();

    // Create a DP table with dimensions (n+1) x (2 * max_weight + 1)
    // Making a multiple of max_weight is ensure that the array indices are non-negative.
    let mut dp = vec![vec![false; (2 * max_weight + 1) as usize]; n + 1];

    // Initialize the value at (0, max_weight) to true, as it represents 0 weight.
    dp[0][max_weight as usize] = true;

    // For each weight in v
    for i in 1..=n {
        let weight = v[i - 1];
        // 각 가중치 값에 대해 현재 인덱스 i까지의 가중치를 사용하여 가중치를 생성할 수 있는지 여부를 확인.
        // If j can be created, mark dp[i][j] as true.
        for j in 0..=(2 * max_weight) {
            // dp[i][j]를 dp[i-1][j]가 true 또는 j >= weight일 때, dp[i-1][j-weight]가 true이면 true로 업데이트함.
            // 이전 인덱스 'i-1'까지의 가중치를 사용하여 가중치를 생성할 수 있는지,
            // 현재 가중치가 현재 가중치 값보다 작거나 같은지 확인하여 이를 수행한다.
            // 두 조건 중 하나라도 true이면 현재 인덱스의 dp 값이 true로 설정된다.
            dp[i][j as usize] = dp[i - 1][j as usize] ||
                (j >= weight && dp[i - 1][(j - weight) as usize]);
            // dp[i][j]를 dp[i][j]가 true 또는 j <= -weight + 2 * max_weight일 때, dp[i-1][j+weight]가 true이면 true로 업데이트함.
            // 마찬가지로 이전 인덱스 'i-1'까지의 가중치를 사용하여 가중치를 생성할 수 있는지,
            // 현재 가중치가 현재 가중치 값보다 크거나 같은지 확인.
            // 두 조건 중 하나라도 true이면 현재 인덱스의 dp 값이 true로 설정된다.
            dp[i][j as usize] = dp[i][j as usize] ||
                (j <= -weight + 2 * max_weight && dp[i - 1][(j + weight) as usize]);
        }
    }

    // For each bead weight in beads
    for b in beads {
        // 마지막으로 프로그램은 max_weight에 구슬의 무게를 더하고
        let weight = b + max_weight;
        let mut res = "N ";
        // 해당 dp 값을 확인하여 각 구슬 무게를 생성할 수 있는지 확인.
        // dp 값이 true이면 프로그램은 "Y "를 출력하고(가중치를 생성할 수 있음을 나타냄)
        // 그렇지 않으면 "N "을 출력한다.
        if weight <= 2 * max_weight && dp[n][weight as usize] {
            res = "Y ";
        }
        write!(output, "{}", res)?;
    }

    Ok(())
}

// // flatten dp
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     let mut output = BufWriter::new(io::stdout().lock());
//     io::stdin().read_to_string(&mut input)?;
//
//     let mut sc = Scanner::new(&input);
//
//     let n = sc.read::<usize>();
//     let mut v: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();
//
//     let m = sc.read::<usize>();
//     let beads: Vec<usize> = (0..m).map(|_| sc.read::<usize>()).collect();
//
//     let max_weight = v.iter().sum::<usize>();
//     let mut dp = vec![false; max_weight+1];
//     dp[0] = true;
//
//     let mut weights = Vec::new();
//     let mut cur_weight = 0;
//
//     for i in 0..n {
//         for j in 0..=cur_weight {
//             let weight = v[i] + j;
//             let weight_abs_diff = v[i].abs_diff(j);
//
//             if dp[j] && !dp[weight] {
//                 weights.push(weight);
//             }
//
//             if dp[j] && !dp[weight_abs_diff] {
//                 weights.push(weight_abs_diff);
//             }
//         }
//         cur_weight += v[i];
//
//         while !weights.is_empty() {
//             dp[weights.pop().unwrap()] = true;
//         }
//     }
//
//     for bead in beads {
//         write!(output,
//                match bead {
//                    bead if bead <= max_weight && dp[bead] => "Y ",
//                    _ => "N ",
//                }
//         )?;
//     }
//
//     Ok(())
// }