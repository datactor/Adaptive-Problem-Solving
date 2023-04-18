// https://www.acmicpc.net/problem/16928

use std::{
    io::{self, prelude::*, BufWriter},
    collections::{VecDeque, HashMap},
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let (n, m) = (sc.read::<usize>(), sc.read::<usize>());
    let mut shortcut = HashMap::new();
    for _ in 0..n+m {
        shortcut.insert(sc.read::<usize>(), sc.read::<usize>());
    }

    if let Some(ans) = bfs(&shortcut) {
        writeln!(output, "{}", ans)?;
    };
    Ok(())
}

fn bfs(shortcut: &HashMap<usize, usize>) -> Option<u8> {
    let mut board = [0u8; 101];

    let mut dq = VecDeque::from([1]);
    while let Some(cur) = dq.pop_front() {
        if cur == 100 {
            return Some(board[cur])
        }

        for dice in 1..=6 {
            let mut next = cur + dice;
            if next <= 100 && board[next] == 0 {
                if let Some(j) = shortcut.get(&next).copied() {
                    next = j;
                }

                if board[next] == 0 {
                    board[next] = board[cur] + 1;
                    dq.push_back(next);
                }
            }
        }
    }
    None
}