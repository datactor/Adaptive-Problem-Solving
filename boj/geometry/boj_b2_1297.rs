use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout());
    io::stdin().read_line(&mut input)?;

    let mut iter = input.split_ascii_whitespace();
    let d = iter.next().unwrap().parse::<usize>().unwrap();
    let h = iter.next().unwrap().parse::<usize>().unwrap();
    let w = iter.next().unwrap().parse::<usize>().unwrap();

    let x = d as f32 / ((h.pow(2) + w.pow(2)) as f32).sqrt();

    writeln!(output, "{} {}", (x*h as f32) as usize, (x*w as f32) as usize)?;

    Ok(())
}