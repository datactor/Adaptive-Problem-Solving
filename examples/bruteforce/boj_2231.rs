use std::io::prelude::*;
use std::io::{self, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut output = BufWriter::new(io::stdout().lock());

    let n = input.trim().parse::<usize>().unwrap();
    let count = n.to_string().chars().count();
    let min = if n > count * 9 {
        n - count * 9
    } else {
        1
    };

    for i in min..n {
        let mut sum = i;
        let digit = i.to_string().chars().map(
            |s| s.to_digit(10).unwrap()).sum::<u32>();
        sum += digit as usize;
        if sum == n {
            writeln!(output, "{}", i).unwrap();
            break
        }
    }
    if output.buffer().len() == 0 {
        writeln!(output, "0").unwrap();
    }
}