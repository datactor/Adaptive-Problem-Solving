// https://www.acmicpc.net/problem/11660
// O(N.pow(2)) // O(N * M 보다 작음)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();
    let n = first_line.next().unwrap();
    let m = first_line.next().unwrap();

    let mut v: Vec<Vec<i32>> = (0..n).map(|_| lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<i32>().unwrap()).collect()
    ).collect();
    let mut arr = vec![vec![0; n+1]; n+1]; // 합을 넣기 위한 2차열 배열 생성

    for i in 1..n+1 {
        for j in 1..n+1 {
            arr[i][j] = arr[i][j-1] + arr[i-1][j] - arr[i-1][j-1] + v[i-1][j-1];
        } //            왼쪽 누적 합   아래쪽 누적 합   누적합의 중복값   현재 위치의 단일 값
    }

    for _ in 0..m {
        let mut pos = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>()).flatten();
        let (x1, y1, x2, y2) =
            (pos.next().unwrap(), pos.next().unwrap(), pos.next().unwrap(), pos.next().unwrap());
        writeln!(output, "{}", arr[x2][y2] - arr[x1-1][y2] - arr[x2][y1-1] + arr[x1-1][y1-1])?
    } // (x2, y2)까지의 누적합에서 (x1-1, y2)까지의 누적합과 (x2, y1-1)까지의 누적합을 제거하고 중복 제거된 (x1-1, y1-1)의 누적합을 더해준다.
    Ok(())
}
