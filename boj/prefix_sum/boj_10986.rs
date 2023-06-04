// https://www.acmicpc.net/problem/10986
// O(N)
// 수학적인 응용력이 필요한 문제. 반례가 없는 간단하고 완벽한 공식을 찾는 것이 관건이다.

// 어떤 수로 나눈 나머지가 같은 두 값 의 차는 어떤 수로 나누어 떨어짐
use std::{
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut v = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();

    let n = v.next().unwrap();
    let m = v.next().unwrap();

    // 나머지 값의 집합. 누적합을 m으로 나눈 나머지가 idx, 값에 +1씩 카운팅
    let mut res: Vec<i64> = vec![0; m];

    let mut total = 0;
    for i in 0..n {
        let a = v.next().unwrap();
        total += a; // 누적합에 a의 값을 더함. 나머지를 구하기 때문에 total들의 집합으로 해야함.
                    // 뒤의 total((A0..Aj).sum())값에 앞의 total((A0..Ai).sum()) 값을 빼면
                    // Ai+...+Aj의 값이 나오기 때문이다.
                    // 여기서 이 값들을 원소로 나머지를 구해 나머지 값을 idx로 하여 res 배열에 카운팅 해 넣는다.
                    // 나머지가 같은 수를 빼면 무조건 나누어 떨어지기 때문에, 나머지가 같은 집합들의 경우의 수를
                    // 계산한다(i * (i-1) / 2). 그 후에 나누어 떨어지는 0번째 카운팅 값을 추가로 더해준다.
        res[total % m] += 1; // 해당 누적합을 m으로 나눴을 때, 나머지에 해당하는 idx의 값에 카운트 +1
    }

    println!(
        "{}",
        res[0] + (0..m).map(|i| res[i] * (res[i] - 1) / 2).sum::<i64>()
    );
    // res[0]: 연속된 구간의 합의 나머지가 0인 구간의 개수
    // 나머지가 같은 숫자의 조합의 개수를 구함(즉 나머지가 같은 수를 빼면 무조건 나누어 떨어지기 때문)
    Ok(())
}
