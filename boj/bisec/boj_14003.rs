// https://www.acmicpc.net/problem/14003
// O(N lgN)

use std::{
    collections::VecDeque,
    error::Error,
    io::{self, prelude::*, BufWriter},
};

struct Scanner<'a> {
    it: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            it: s.split_ascii_whitespace(),
        }
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse::<T>().ok().unwrap()
    }
}

struct Lis {
    arr: Vec<i32>,
    len_and_val: Vec<(usize, i32)>,
}

impl Lis {
    fn new() -> Self {
        Lis {
            // 최소값을 0번째 idx에 넣고 시작
            arr: vec![-1_000_000_001],
            len_and_val: vec![(0, -1_000_000_001)],
        }
    }

    // v는 입력값을 받은 vecDeque
    fn cal_bisec(&mut self, mut v: VecDeque<i32>) {
        while !v.is_empty() {
            // v의 맨 앞부터 순차 pop
            let num = v.pop_front().unwrap();

            // v.pop()이 arr의 마지막 숫자보다 크다면 Lis.arr, Lis.val_and_len에 차례대로 push
            // Lis.val_and_len에는 val과 arr의 이전 len을 push함(이미 최소값을 넣어놨기 때문에 편의상)
            if &num > self.arr.last().unwrap() {
                self.len_and_val.push((self.arr.len(), num));
                self.arr.push(num);
            } else {
                // v.pop이 arr.last보다 크지 않으면 binary search 실행.
                // bisec을 실행해서 num이 들어가기 적절한 위치를 찾음(lis.arr에서 num보다 작은 수 중 최대값 바로 뒤)
                let idx = {
                    let mut low = -1;
                    let mut high = self.arr.len() as i32;

                    while low + 1 < high {
                        let mid = (low + high) / 2;
                        if &num > &self.arr[mid as usize] {
                            low = mid as i32
                        } else {
                            high = mid
                        }
                    }
                    high as usize
                };

                // 위치를 찾으면 arr[idx]의 값을 바꾸고 val_and_len에 push(중간 값이 바뀌었는데 바뀐 집합이 lis가 아니더라도
                // 상관없다. 중간값이 바뀌어도 arr의 last값과 idx는 변함없기 때문.)
                self.arr[idx] = num;
                self.len_and_val.push((idx, num));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let n = scanner.read::<usize>();
    let v: VecDeque<i32> = (0..n).map(|_| scanner.read()).collect();

    let mut lis = Lis::new();

    lis.cal_bisec(v);

    let mut result_len = lis.arr.len() - 1; // 최소값 하나를 넣고 시작했기 때문에 빼줌
    writeln!(output, "{}", result_len)?;
    let mut result = VecDeque::new();

    // 아쉽게 rust String은 push_front가 없다(내가 모르는 걸지도) 그래서 result를 새로 만들었음.
    // 중간값을 변경하면서 왔기 때문에 역순으로 찾아야 lis 값들이 정확하게 나온다
    while !lis.len_and_val.is_empty() && result_len > 0 {
        let (idx, num) = lis.len_and_val.pop().unwrap();
        if idx == result_len {
            result.push_front(num);
            result_len -= 1;
        }
    }

    result
        .iter()
        .for_each(|s| write!(output, "{} ", s).unwrap());

    Ok(())
}
