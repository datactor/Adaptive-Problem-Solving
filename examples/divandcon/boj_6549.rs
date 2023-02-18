// https://www.acmicpc.net/problem/6549
// O(N)에 가깝다
// https://hooongs.tistory.com/330 레퍼런스 참조


use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    loop {
        let mut f = lines.next().unwrap().split_ascii_whitespace()
            .map(|s| s.parse::<usize>()).flatten();
        let n = f.next().unwrap();
        if n == 0 {
            break
        }
        let v: Vec<usize> = f.collect();
        let mut result = 0;

        let mut dq = VecDeque::new(); // dq의 값은 v의 idx를 저장시킴
        let mut width = 0;
        for i in 0..n {
            while !dq.is_empty() && v[*dq.back().unwrap()] > v[i] { // dq가 비어있지 않고, v[dq의 마지막 값(마지막에 추가된 idx)]가 v[i]보다 클 경우
                let tmp = dq.pop_back().unwrap(); // dq의 마지막 값(idx)을 dq에서 빼고 tmp에 저장
                println!("{} {}", i, tmp);

                if dq.is_empty() { // dq의 last 값을 pop했을 때, dq가 비어있으면
                    width = i; // result와 이전의 값 * width를 비교한다.
                } else { // dq가 비어있지 않으면
                    width = i - dq.back().unwrap() - 1; // pop을 해놓은 상태에서 마지막 idx를 구함
                } result = usize::max(result, width * v[tmp]);
                // println!("{:?}, {}, {}, {}", dq, width, result, tmp);
            }
            dq.push_back(i); // dq가 비어있을 때 idx를 dq에 추가
        }
        // dq에 저장된 idx를 역순으로 다시 max 계산
        while !dq.is_empty() {
            let tmp = dq.pop_back().unwrap();

            if dq.is_empty() {
                width = n;
            } else {
                width = n - dq.back().unwrap() - 1;
            } result = usize::max(result, width * v[tmp])
        }

        writeln!(output, "{}", result)?;
    }

    Ok(())
}