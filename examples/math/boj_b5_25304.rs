use std::{
    io::{self, prelude::*, BufReader, BufWriter},
};

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input)?;
    let mut x = input.trim().parse::<i32>().unwrap();

    input.clear();
    reader.read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input)?;
        let v: Vec<_> = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        x -= v[0] * v[1];
    }

    writeln!(writer, "{}", if x == 0 {
        "Yes"
    } else {
        "No"
    })?;

    Ok(())
}