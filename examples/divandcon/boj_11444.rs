use std::{
    io::{self},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n = input.trim().parse::<u128>().unwrap();
    let x = [[1, 1], [1, 0]];

    let ans = solve(x, n);

    println!("{}", ans[0][1] % 1000000007);
    Ok(())
}

fn mul(x: &[[usize; 2]; 2], mat: &[[usize; 2]; 2]) -> [[usize; 2]; 2] {
    let mut result = [[0; 2]; 2];
    for r in 0..2 {
        for i in 0..2 {
            result[r][i] = (x[r].iter().enumerate()
                .map(|(j, num)| num * mat[j][i]).sum::<usize>()) % 1000000007;
        }
    } result
}

fn solve(x: [[usize; 2]; 2], n: u128) -> [[usize; 2]; 2] {
    if n == 1 {
        return x
    } else {
        let tmp = solve(x, n/2);
        if n % 2 == 0 {
            return mul(&tmp, &tmp)
        } else {
            return mul(&mul(&tmp, &tmp), &x)
        }
    }
}