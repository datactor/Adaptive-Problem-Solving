// https://www.acmicpc.net/source/54993028
// 이진트리 문제이지만 HashMap을 사용하면 훨씬 쉽고 속도도 빠르다.

use std::{
    collections::HashMap,
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.lines();
    input.next();

    let mut deck = HashMap::new();
    input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|a| {
            *deck.entry(a).or_insert(0) += 1;
            // if let Some(t) = deck.get(&a) {
            //     deck.insert(a, t + 1);
            // } else {
            //     deck.insert(a, 1);
            // }
        });
    input
        .skip(1)
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|v| writeln!(output, "{}", deck.get(&v).unwrap_or(&0)).unwrap());

    Ok(())
}
