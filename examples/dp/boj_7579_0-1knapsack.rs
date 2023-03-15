// https://www.acmicpc.net/problem/7579
// O(n.pow(2))

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

// 목표는 총 중량(메모리 합)을 m이하로 유지하면서 총 가치(cost 합)를 최대화 하는 것
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let (n, m) = (sc.read::<usize>(), sc.read::<usize>());
    // 각 앱에 필요한 메모리를 가중치로 정의
    let mem: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();
    // 각 앱을 비활성화 하는 비용을 값으로 정의
    let costs: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();

    // 0-1 knapsack table 정의. 인덱스는 cost를 나타내고,
    // 해당 인덱스의 값은 해당 비용으로 해제할 수 있는 최대 메모리를 나타냄
    let mut dp = [0; 10001];
    dp[costs[0]] = mem[0];

    // 위에서 인덱스 0에 대한 기본 사례를 이미 설정했기 때문에 인덱스 1에서부터 시작함.
    for i in 1..n {
        let cur_cost = costs[i];
        // dp 배열을 끝(높은 비용)에서 처음(낮은 비용)까지 반복한다.
        for j in (0..10001).rev() {
            // dp[j]가 0이라면 메모리가 0이라는 뜻으로, 비활성화해도 메모리 사용량에 영향을 미치지 않으므로
            // 다음 반복을 계속한다.(정의되지 않은 cost-memory pair이거나 메모리 사용량이 0)
            if dp[j] == 0 && j > 0 {
                continue
            }
            println!("{}", j);
            // 새 비용 k는 cur_cost + j로 계산한다.
            // 여기서 cur_cost는 현재 앱을 비활성화하는 비용이고, j는 현재 비용이다.
            // 즉 k에서 cur_cost는 mem[i]의 비활성화 비용, j는 dp[j]의 비활성화 비용이다.
            let k = cur_cost + j;
            // dp[k]를 업데이트한다. 값이 이미 존재할 경우, 기존 값과 비교해서 더 큰 값을 dp[k]에 할당한다.
            // 여기서 dp[j]는 이전의 앱 비활성화되는 메모리, mem[i]는 추가되는 비활성화 메모리이다.
            if dp[k] != 0 {
                dp[k] = usize::max(dp[j] + mem[i], dp[k]);
            } else {
                dp[k] = dp[j] + mem[i]
            }
        }
    }
    // 이것을 반복하고 나면 비용당 최대의 메모리 비활성도를 가진 테이블이 완성된다.

    writeln!(output, "{}", dp.iter().position(|&cost| cost >= m).unwrap())?;

    Ok(())
}