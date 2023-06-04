// https://www.acmicpc.net/problem/16931
// O(n * m)

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(io::stdout().lock());
    let mut line = || input.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let first_line = line();

    let mut area = first_line[0] * first_line[1] * 2;
    let v = (0..first_line[0]).map(|_| {
        let vec = line();
        let len = vec.len();
        area += vec[0] + vec[len-1];
        for i in 0..len-1 {
            area += (vec[i] - vec[i+1]).abs()
        }
        vec
    })
        .collect::<Vec<Vec<i32>>>();

    area += transpose_matrix(&v);

    writeln!(output, "{}", area)?;

    Ok(())
}

fn transpose_matrix(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut area = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut vec = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            vec[j][i] = matrix[i][j];
        }
    }

    for v in &vec {
        let len = v.len();
        area += v[0] + v[len-1];
        for i in 0..len-1 {
            area += (v[i] - v[i+1]).abs()
        }
    }

    area
}