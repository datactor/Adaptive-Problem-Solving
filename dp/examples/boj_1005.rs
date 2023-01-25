// https://www.acmicpc.net/problem/1005

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

    let t = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut f_line = lines.next().unwrap().split_ascii_whitespace();
        let (n, k) = (f_line.next().unwrap().parse::<usize>().unwrap(),
                      f_line.next().unwrap().parse::<usize>().unwrap());
        let build_times: Vec<usize> = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>().unwrap()).collect();

        // node(idx)로 진입 가능한 node 개수를 세기 위한 list
        let mut node_entrances_num = vec![0; n+1];
        // nodes to nodes (list)
        let mut nodes = vec![Vec::new(); n+1];

        for _ in 0..k {
            let mut tmp = lines.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<usize>()).flatten();
            let (x, y) = (tmp.next().unwrap(), tmp.next().unwrap());
            nodes[x].push(y);
            node_entrances_num[y] += 1;
        }

        let building_num = lines.next().unwrap().parse::<usize>().unwrap();

        // 해당 idx의 node로 접근하기 위해 걸리는 최단 시간을 넣기 위한 dp
        let mut dp = vec![0; n+1];

        let mut q: VecDeque<usize> = node_entrances_num.iter().enumerate().skip(1).filter_map(
            // 시작점 찾기. node로 진입 가능한 점이 없으면(0이면) 시작점. 여러개 있을 수 있음
            |(idx, &v)| (v == 0).then(|| {
                dp[idx] = build_times[idx-1]; // 시작 node의 건설시간을 dp에 저장
                idx // 시작 node의 idx를 q에 저장
            })).collect();

        while !q.is_empty() {
            let a = q.pop_front().unwrap(); // 건설되는 빌딩(idx)
            for i in &nodes[a] { // 건설되는 빌딩 다음에 지을 수 있는 빌딩 모두 불러옴
                node_entrances_num[*i] -= 1; // 불러온 빌딩들을 지을 것이므로 필요선행 빌딩 갯수 하나 제거
                // (i빌딩 건설시간 + 선행빌딩 건설시간 vs 이미 저장되어 있는 i빌딩 건설하는데까지 걸린 총 시간) 중 큰 수를 dp[i]에 저장
                dp[*i] = std::cmp::max(dp[a] + build_times[*i-1], dp[*i]);
                // 필요선행 빌딩이 없을 경우 q에 넣어 건설 예정으로 정함.
                if node_entrances_num[*i] == 0 {
                    q.push_back(*i);
                }
            }
        }

        writeln!(output, "{}", dp[building_num])?;
    }

    Ok(())
}