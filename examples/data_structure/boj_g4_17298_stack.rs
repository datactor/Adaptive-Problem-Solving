// https://www.acmicpc.net/problem/17298
// 최악의 경우 O( n(n-1)/2 ) 이지만,
// 스택요소들의 순서를 고려하지 않고 중간에 break할 수 있게하여 최적화.

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let input = input.split_ascii_whitespace();
    let a: Vec<i32> = input.skip(1).into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let n = a.len();

    // 초기값 설정
    let mut stack = Vec::with_capacity(n);
    let mut ans = vec![-1; n];

    // 마지막부터 오른쪽에 큰 수가 있는지 탐색
    for i in (0..n).rev() {
        while let Some(&last) = stack.last() {
            if last > a[i] {
                ans[i] = last;
                break
            } else {
                // 가까운 오른쪽의 수 (last)가 a[i]보다 작거나 같을 경우, pop하고 더 멀리 있는 오른쪽 수 중에 찾는다.
                stack.pop();
            }
        }
        // lifo로 순서를 고려하지 않기 위해 여기서 stack.push.
        // 가까운 오른쪽의 수일 수록 마지막에 push되고 멀리 떨어질 수록 안쪽으로 밀리기 때문에,
        // stack의 끝자리에 있을 수록 더 오른쪽에 가까운 수임을 보장함.
        // 그러므로 last가 a[i]보다 더 큰 순간 break로 나와도 됨.
        stack.push(a[i]);
    }

    for i in ans {
        write!(output, "{} ", i)?;
    }

    Ok(())
}