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

    println!("{:?}", v);
    println!("{:?}", blank);

    // black에 있는 튜플을 idx 순서대로 탐색
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
        output.flush().unwrap();
        std::process::exit(0); // 스도쿠를 하나라도 완료되면 프로세스 종료.
    }

    for i in 1..10 {
        let (x, y) = (blank[idx].0, blank[idx].1);

        // row, col, sqr 모두를 계산해봐서 v[x][y] 자리에 i가 들어 갈 수 있으면, v[x][y]를 i로 놓고
        // blank의 다음 자리에 있는 0에 대해 dfs를 실행함.
        // dfs가 끝나면, 제자리에 다시 0을 반환하고 다음 숫자(i)에 대해 dfs 실행
        if check_row(x, i, v) && check_col(y, i, v) && check_sqr(x, y, i, v) {
            v[x][y] = i;
            dfs(idx + 1, blank, v, output);
            v[x][y] = 0;
        }
    }
}