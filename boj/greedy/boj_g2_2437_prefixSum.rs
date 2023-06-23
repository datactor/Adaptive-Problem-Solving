// https://www.acmicpc.net/problem/2437
// O(n lg n)

// ref https://4legs-study.tistory.com/97
// 주어진 무게추로 측정할 수 없는 최소무게를 구하는 문제이다.
//
// 무게추 1, 2, 3을 갖고 있다고 가정할 때, 1에서 6까지의 모든 무게를 측정할 수 있다.
// 이 상태에서 새로운 무게추 5가 추가되었을 때, 이미 우리는 1에서 6까지의 무게를 1~3 무게추로 측정할 수 있었기 때문에
// 각 무게에 대한 무게추 조합에 5 무게추만 추가한 새로운 조합을 만들어 낼 수 있다.
//
// 다시 말해 우리가 2, 3 무게추를 이용해 5의 무게를 측정할 수 있다면,
// 이 조합에 5 무게추만 추가해 5 + 5 = 10 의 무게를 측정할 수 있다는 것이다.
// 따라서 측정할 수 있는 무게의 범위는 1 ~ 11이 된다.
//
// 하지만 만약 1, 2, 3 무게추를 갖고 있을 때 새로운 무게추 8이 추가된다면 어떻게 될까?
// 1, 2, 3 무게추로 측정할 수 있는 최대 무게는 6이지만,
// 1, 2, 3에 새 무게추 8을 더해 측정할 수 있는 최소 무게는 8이기 때문에 우리는 7의 무게를 측정할 수 없게 된다.
//
// 이를 통해 다음 사실을 알 수 있다.
//
// "현재까지 측정 가능한 [최대 무게 + 1]보다 큰 무게추가 추가된다면 측정할 수 없는 무게가 발생한다."
//
// 따라서 주어진 무게추들을 오름차순으로 정렬한 후,
// 앞까지의 누적합보다 2 이상 큰 무게추가 등장했을 경우가 측정할 수 없는 무게가 나타난다.

use std::{
    io::{self, Read, Write, BufWriter},
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

    fn next<T>(&mut self) -> Result<T, Box<dyn Error>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
    {
        self.input.next()
            .ok_or("Reached end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

macro_rules! ok {
    (()) => {
        {
            let mut buf_writer = BufWriter::new(io::stdout().lock());
            let mut buffer = String::new();
            io::stdin().lock().read_to_string(&mut buffer)?;

            let mut scanner = Scanner::new(&buffer);
            let n = scanner.next::<usize>()?;
            let mut weight = (0..n).map(|_| scanner.next::<i32>().unwrap()).collect::<Vec<i32>>();
            weight.sort_unstable();

            let mut mn = 0;
            for w in weight {
                if w <= mn + 1 {
                    mn += w;
                } else {
                    break
                }
            }
            write!(buf_writer, "{}", mn+1)?;

            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    ok!(())
}