// https://www.acmicpc.net/problem/3015
// O(n)

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let heights: Vec<usize> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|p| p.parse::<usize>().unwrap())
        .collect();

    let mut stack = Vec::new();
    let mut ans = 0;

    for h in heights {
        // 스택에 사람들이 있는지 반복하여 확인.
        while let Some((prev_h, prev_cnt)) = stack.pop() {
            if prev_h < h { // 스택에서 꺼낸 사람이 현재 사람보다 키가 작으면, ans에 그 수를 추가.
                ans += prev_cnt as u64;
            } else { // 스택에서 꺼낸 사람이 키가 크면, 다시 스택에 넣고 break
                stack.push((prev_h, prev_cnt));
                break;
            }
        }

        let mut cnt = 1;

        // 키가 같은 사람을 찾기 위해 다시 스택에 사람이 있는지 확인함
        if let Some((prev, prev_cnt)) = stack.last_mut() {
            // 스택에 있으면, 꺼내서 현재 사람과 키가 같은지 확인. 같다면 현재 볼 수 있는 숫자(cnt)에 더하고,
            // ans에도 더함. 업데이트 했으니 스택에서 지운다.
            if *prev == h {
                cnt += *prev_cnt;
                ans += *prev_cnt as u64;
                stack.pop();
            }
        }

        if !stack.is_empty() {
            ans += 1;
        }

        stack.push((h, cnt));
    }

    writeln!(output, "{}", ans)?;
    Ok(())
}