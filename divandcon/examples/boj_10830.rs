use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut f_line = lines.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<usize>()).flatten();
    let (n, b) = (f_line.next().unwrap(), f_line.next().unwrap());

    let mut mat: Vec<Vec<usize>> = (0..n)
        .map(
            |_| lines.next().unwrap().split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap()).collect()
        )
        .collect();

    let result = solve(n, b, mat);
    for i in result {
        for j in i {
            write!(output, "{} ", j % 1000)?;
        } writeln!(output)?;
    }

    Ok(())
}

fn mul(mut x: Vec<Vec<usize>>, mat: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; n]; n];
    for r in 0..n {
        for i in 0..n {
            result[r][i] = (x[r].iter().enumerate().map(|(j, num)| num * mat[j][i]).sum::<usize>()) % 1000;
        }
    } result
}

fn solve(n: usize, b: usize, x: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    if b == 1 {
        return x
    } else if b == 2 {
        return mul(x.clone(), &x, n)
    } else {
        let mut tmp = solve(n, b/2, x.clone()); // 새로운 stack frame 생성, 완료되면 결과를 반환하고, 스택프레임이 popped off됨.
        if b % 2 == 0 {
            return mul(tmp.clone(), &tmp, n)
        } else {
            return mul(mul(tmp.clone(), &tmp, n), &x, n)
        }
    }
}