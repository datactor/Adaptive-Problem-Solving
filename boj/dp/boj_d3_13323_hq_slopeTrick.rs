// https://www.acmicpc.net/problem/13323
// ref: https://velog.io/@idwooin/Slope-Trick

/// 논리:
/// A수열 내의 숫자가 sorting이 되지 않은 채로 존재.
/// B수열은 오름차순 순열이다. Ai와 Bi의 차이를 가장 작게 하는 B수열을 구해보자.
///
/// A수열을 B수열로 만드는 것은 결국 A수열의 각 숫자를 적절한 위치로 옮기는 것과 같다.
/// 옮겨지는 거리나 방향은 원래 위치에서 목표 위치까지의 거리에 따라 달라진다.
/// 만약 숫자가 원래 위치에서 오른쪽으로 이동해야 한다면, 그 숫자는 증가한다.
/// 반대로, 숫자가 왼쪽으로 이동해야 한다면, 그 숫자는 감소해야 한다.
/// 이렇게 숫자가 증가하거나 감소하는 횟수가 바로 A 수열에서 B 수열을 만드는 데 필요한 최소 증감 횟수이다.
///
/// 즉, A수열 안의 각각 숫자를 1씩 증감시켜서 B수열을 만드는 최소 숫자 증감 횟수를 구하는 것과 같다.
///
/// dp(i, k) = 배열의 i번째 위치의 값이 k일 때, 배열의 1..i번째 숫자를 오름차순으로 만들기 위한 최소 연산 횟수
/// Ai = 처음 주어진 배열의 i번째 위치 값
/// dp(i, k) = min(dp(i-1, k-1), dp(x-1, k-2),...,dp(x-1, 1)) + |Ai - k|
/// - 위에서 |Ai-k|는 내 위치의 값을 k로 만들기 위한 연산 횟수이고,
/// - min(dp(i-1, k-1), dp(x-1, k-2),...,dp(x-1, 1))는 배열에서 내 바로 앞 위치까지의 값은
///   전부 오름차순이면서 k보다 작은 값이어야 하기 때문이다.
/// 즉, dp(i-1, k-1), dp(x-1, k-2),...,dp(x-1, 1) 중에서 연산 횟수가 최소로 드는 것을 고른 후,
/// |Ai - k|를 더한 것과 같다.
///
/// Slope Trick는 y = f(x) 형태의 함수 그래프의 기울기를 사용하여 문제를 해결하는 알고리즘이다.
/// 주어진 수열 A를 정렬된 수열 B로 바꾼다면 각 idx를 x축 상의 점으로 생각하고,
/// 그 위치에 해당하는 수열 A의 숫자를 y좌표로 사용하여 점을 찍는다.
/// 이렇게 찍힌 점들은 무작위 순서로 배치되어 있다.
///
/// A = [9, 4, 8, 20, 14, 15, 18] 라고 정의해보자. 그래프는 아래와 같다.
/// 20 -              x
/// 19 -
/// 18 -                          x
/// 17 -
/// 16 -
/// 15 -                      x
/// 14 -                  x
/// 13 -
/// 12 -
/// 11 -
/// 10 -
/// 9  -  x
/// 8  -          x
/// 7  -
/// 6  -
/// 5  -
/// 4  -      x
/// 3  -
/// 2  -
/// 1  -
/// 0  -  -------------------------------------
///       0   1   2   3   4   5   6   7   8   9
///
/// 이 점들을 정렬된 순서대로 만들어 보자.
/// 즉, 왼쪽에서 오른쪽으로 이동하면서 y값이 증가하는 형태로 만들어야 한다.
/// 각 점이 이동하는 거리는 그 점의 y값과 x값의 차이로 계산할 수 있다. 여기서 x값은 각 점의 목표 위치를 의미한다.
/// 그렇다면 이 이동을 최소화하는 방법은?
/// 여기서 `Slope Trick`을 사용한다. `Slope Trick`은 이동해야 하는 거리를 최소화하는 방향을 찾는 방법이다.
/// 방법은 각 점에서 이동해야 하는 거리를 계산하고, 이 거리가 가장 작은 점을 먼저 이동시킨다.
/// ```rust
/// let required_movement = scanner.next::<i32>()? - i;
/// // 여기서 required_movement는 각 점이 이동해야 하는 거리를 의미한다. 이 거리는 현재 idx와 그 idx의 목표 위치의 차이로 계산된다.
/// ```
///
/// 그 다음으로, 이동해야 하는 거리가 가장 작은 점을 먼저 이동시키는 부분은 다음과 같다.
/// ```rust
/// if !hq.is_empty() && *hq.peek().unwrap() > required_movement { // 만약 required_movement보다 peek이 크면, required_movement보다 더 많은 움직임이 필요하다. 그렇기 때문에 if 조건문에서 미리 peek().unwrap()으로 제거한다.
///     // 따라서 우리는 더 큰 움직임이 필요한 요소를 제거했으며,
///     // 이것을 더 최적으로 처리하기 위해 현재 `required_movement`를 hq에 push하고,
///     hq.push(required_movement);
///     // 현재의 hq중 가장 큰 값을 pop하여 이 최적의 움직임을 res에 더하여 기록한다.
///     res += (hq.pop().unwrap() - required_movement) as i64;
///
///     // 위의 코드는 이전의 위치를 변경하기 위한 움직임을 최적으로 처리한 것이고,
///     // 아직 현재의 required_movement를 처리하지 않았기 때문에 다시 push한다.
///     // 이 required_movement는 이후에 처리될 것이다.
///     hq.push(required_movement);
/// } else {
///     hq.push(required_movement);
///     // 만약 peek이 required_movement보다 작거나 같다면,
///     // 처리하지 않고 현재의 required_movement를 push한다(추후에 처리될 것임).
///     // 이는 peek이 required_movement를 초과하는 움직임이 필요한 요소가 아니기 때문이다.
/// }
/// ```
/// hq의 최대 힙에 이동해야 하는 거리`required_movement`를 저장하고, `hq.peek().unwrap()`으로 이동해야 하는
/// 거리 중, 가장 큰 값을 반환한다. `hq.peek().unwrap()`이 `required_movement`보다 크다면, `peek`값을 먼저 이동 시킨다.
/// 이렇게 하면 이동해야 하는 총 거리를 최소화할 수 있다.
use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    collections::BinaryHeap,
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

    fn next<T> (&mut self) -> Result<T, Box<dyn Error>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
    {
        self.input.next()
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<i32>()?;

    let mut min_operations = 0;
    let mut hq = BinaryHeap::with_capacity(n as usize);
    for cur_idx in 0..n {
        let required_movement = scanner.next::<i32>()? - cur_idx;
        // 현재 위치에서 이동해야하는 방향과 거리.
        // 우리의 목표는 주어진 배열을 정렬된 배열로 만드는 것이다.
        // 즉, 정렬되지 않은 A배열의 현재 위치에서 정렬된 시퀀스의 올바른 위치에 도달하도록 해야한다.
        // 이때, 각 위치를 옮기는 횟수를 최소화하는 것이 중요하다.
        // 그래서 각 위치에서 이동해야 하는 거리와 방향을 알아야함.
        //
        // 배열을 정렬하려면, 각 위치가 원래 위치에서 다른 위치로 이동해야 한다.
        // 이동해야 하는 거리는 현재 위치와 목표 위치 사이의 차이에 의해 결정된다.
        // "현재 위치 - i"는 현재 위치가 원래 있어야 할 위치와의 차이이다.

        if !hq.is_empty() && *hq.peek().ok_or("Empty Heap")? > required_movement {
            // heap queue를 확인하여, 만약 큐의 가장 큰 값(가장 위에 있는 값)이 required_movement보다 크다면,
            // 이 값은 더 작은 값으로 변경되어야 함. 이는 숫자가 왼쪽으로 이동해야 함을 의미.
            // 따라서 큐의 가장 큰 값을 제거하고, 그에 해당하는 움직임을 최적으로 하기 위해 required_movement에 push후
            // hq의 가장 큰 요소를 추출하여 res에 더한 다음, 현재 required_movement를 처리하기 위해 큐에 다시 추가.
            hq.push(required_movement);
            min_operations += (hq.pop().ok_or("Empty Heap")? - required_movement) as i64;
            hq.push(required_movement);
        } else {
            // 그렇지 않다면, 즉 required_movement가 큐의 가장 큰 값보다 크거나 같다면, 이 숫자는 그대로 둘 수 있다.
            // 이 경우 비용은 발생하지 않는다.
            hq.push(required_movement);
        }
        // 이 방식으로 각 숫자가 이동해야 하는 방향과 거리를 계산하고, 그에 따라 필요한 증감 횟수를 계산한다.
        // 이는 결국 배열을 정렬하는 데 필요한 최소 증감 횟수를 구하는 문제를 해결하는 것이다.
    }

    write!(buf_writer, "{}", min_operations)?;
    Ok(())
}