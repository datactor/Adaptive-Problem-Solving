// https://www.acmicpc.net/problem/1874

use std::{
    error::Error,
    fmt::Write,
    io::{stdin, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let v: Vec<usize> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut a = Vec::new();

    let mut x = 0;
    input.clear();
    for i in 0..v.len() {
        while x < v[i] {
            x += 1;
            a.push(x);
            writeln!(input, "+").unwrap();
        }
        if a.pop() == Some(v[i]) {
            writeln!(input, "-").unwrap();
        } else {
            input.clear();
            writeln!(input, "NO").unwrap();
            break;
        }
    }
    print!("{}", input);
    Ok(())
}
