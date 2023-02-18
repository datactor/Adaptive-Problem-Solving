// https://www.acmicpc.net/problem/2580

use std::{
    io::{self, prelude::*, BufWriter, StdoutLock},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut blank = Vec::new();
    let mut v = (0..9)
        .map(|i| {
            let line: Vec<usize> = lines.next().unwrap()
                .split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            (0..9).for_each(|j| if line[j] == 0 { blank.push((i, j)) });
            line
        })
        .collect();

    // blank에 있는 튜플을 idx 순서대로 탐색
    dfs(0, &blank, &mut v, &mut output);

    Ok(())
}

// 빈 숫자에 a가 들어갈 수 있으면 true 리턴
fn check_row(x: usize, a: usize, v: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        if a == v[x][i] {
            return false
        }
    } true
}

fn check_col(y: usize, a: usize, v: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        if a == v[i][y] {
            return false
        }
    } true
}

fn check_sqr(x: usize, y: usize, a: usize, v: &Vec<Vec<usize>>) -> bool {
    let (nx, ny) = (x / 3 * 3, y / 3 * 3);
    for i in 0..3 {
        for j in 0..3 {
            if a == v[nx+i][ny+j] {
                return false
            }
        }
    } true
}

fn dfs(idx: usize, blank: &Vec<(usize, usize)>, v: &mut Vec<Vec<usize>>, output: &mut BufWriter<StdoutLock>) {
    if idx == blank.len() {
        for i in 0..9 {
            for j in 0..9 {
                write!(output, "{} ", v[i][j]).unwrap();
            }
            write!(output, "\n").unwrap();
        }

        output.flush().unwrap(); // process를 강제 종료하면 write가 완료되지 않고 끝나기 때문에 flushing 해줌.
                                 //
                                 // return으로 종료하지 않는 이유? 여러 dfs중 하나만 종료되고 다른 dfs는 진행되어
                                 // 추가적인 스도쿠를 완료할 수 도 있음
        std::process::exit(0); // 스도쿠를 하나라도 완료하면 프로세스 종료.
    }
    let (x, y) = (blank[idx].0, blank[idx].1);

    for i in 1..10 {
        // 1. row, col, sqr 모두를 탐색해보고 v[x][y]자리에 들어갈 수 있는 i를 찾는다(for i in 1..10).
        //    i를 찾으면, v[x][y]의 값을 i로 바꾸고, 다음 dfs(blank의 다음 자리: idx+1)를 실행한다.
        // -- 반복 --
        //
        // 2. dfs가 끝나면(모두 true를 반환하는 i가 없으면),
        //    후입 선출 순서로 마지막 dfs의 제자리에 0을 반환하고 이전 idx로 돌아가서 for문을 마저 돈다( -> 1).
        if check_row(x, i, v) && check_col(y, i, v) && check_sqr(x, y, i, v) {
            v[x][y] = i;
            dfs(idx + 1, blank, v, output);
            v[x][y] = 0;
        }
    }
}