use std::io::prelude::*;
use std::io::{self, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut output = BufWriter::new(io::stdout().lock());

    let n = input.trim().parse::<usize>().unwrap();
    let mut array = vec![b' '; n * (n + 1)];
    for i in 0..n {
        array[i * (n + 1) + n] = b'\n';
    }
    solve(&mut array, 0, 0, n, n);
    write!(output, "{}", std::str::from_utf8(&array).unwrap()).unwrap();
}

fn solve(array: &mut Vec<u8>, row: usize, col: usize, size: usize, len: usize) {
    println!("x: {}, y: {}, z: {}, len: {}, point: {}, {}", row, col, size, len, (row * (len + 1) + col)/(len+1), (row * (len + 1) + col)%(len+1));
    if size == 1 {
        array[row * (len + 1) + col] = b'*';
    }
    else {
        for i in 0..3 {
            for j in 0..3 {
                if i != 1 || j != 1 {
                    let tmp = size / 3;
                    solve(array, i * tmp + row, j * tmp + col, tmp, len);
                }
            }
        }
    }
}