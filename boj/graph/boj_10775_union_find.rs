// https://www.acmicpc.net/problem/10775
// union-find
// O(N * lgN)

use std::{
    error::Error,
    io::{self, prelude::*},
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner {
        Scanner {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn find(x: usize, parents: &mut Vec<usize>) -> usize {
    if x == parents[x] {
        return x;
    }
    parents[x] = find(parents[x], parents);
    return parents[x];
}

fn union(mut x: usize, mut y: usize, parents: &mut Vec<usize>) {
    // 앞자리와 연결
    (x, y) = (find(x, parents), find(y, parents));
    parents[y] = x;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (g, p) = (scanner.read::<usize>(), scanner.read::<usize>());

    let mut gates: Vec<usize> = (0..g + 1).map(|i| i).collect(); // 예를들어 [0]=0, [1]=1, [2]=2, [3]=3, [4]=4의 게이트 생성
    let mut cnt = 0; // (1번 인덱스는 1번자리에 들어가면 된다는 뜻)

    for _ in 0..p {
        let gi = scanner.read::<usize>();
        let gate = find(gi, &mut gates); // gates의 gi부터 자리값과 idx가 같은지 찾기(찾을때까지 재귀를 돌린다)
        if gate == 0 {
            break;
        }
        union(gate - 1, gate, &mut gates); // gates[gi]의 값에 gates[gi-1]의 값을 넣음(하나의 비행기가 들어가서 자리를 채웠으니 채우고 남은 자리의 최후방 자리를 값으로 취함)
        cnt += 1;
    }

    println!("{cnt}");

    Ok(())
}
