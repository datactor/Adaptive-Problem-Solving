// https://www.acmicpc.net/problem/1920
// 이분탐색이 아니라 control group을 hashset으로 O(1)로 색인함

use std::{
    collections::HashSet,
    error::Error,
    io::{self, Write, BufRead, BufReader, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = BufReader::new(io::stdin().lock()).lines();
    let mut output = BufWriter::new(io::stdout().lock());
    lines.next();

    let n_list: HashSet<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    lines
        .skip(1)
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .for_each(
            |s| {
                let i = s.parse::<i32>().unwrap();
                writeln!(
                    output,
                    "{}",
                    match n_list.contains(&i) {
                        true => 1,
                        false => 0,
                    }
                ).unwrap()
            }
        );

    Ok(())
}