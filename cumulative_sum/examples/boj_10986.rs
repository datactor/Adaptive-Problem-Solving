// https://www.acmicpc.net/problem/10986
// O(N)
// 수학적인 응용력이 필요한 문제. 어떤 수로 나눈 나머지가 같은 두 값 의 차는 어떤 수로 나누어 떨어짐

use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v = input.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = v.next().unwrap();
    let m = v.next().unwrap();

    // 나머지 값의 집합. 누적합을 m으로 나눈 나머지가 idx, 값에 +1씩 카운팅
    let mut res: Vec<i64> = vec![0; m];

    let mut total = 0;
    for _ in 0..n {
        let a = v.next().unwrap();
        total += a; // 누적합에 a의 값을 더함
        println!("{}", total);
        res[total % m] += 1; // 해당 누적합을 m으로 나눴을 때, 나머지에 해당하는 idx의 값에 카운트 +1
    }

    println!("{:?}", res);
    println!("{}", res[0] + (0..m).map(|i| res[i] * (res[i] - 1) / 2).sum::<i64>());
    // res[0]: 연속된 구간의 합의 나머지가 0인 구간의 개수
    // 나머지가 같은 숫자의 조합의 개수를 구함(즉 나머지가 같은 수를 빼면 무조건 나누어 떨어지기 때문)
    Ok(())
}