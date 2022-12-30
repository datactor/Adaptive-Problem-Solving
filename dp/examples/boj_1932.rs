// https://www.acmicpc.net/source/53250332

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();

    let num_rows = lines.next().unwrap().parse::<usize>()?;
    let first_row = lines.next().unwrap().parse::<usize>()?;

    let mut arr = vec![first_row];

    for _ in 1..num_rows {
        let v = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let mut tmp = Vec::new();
        for j in 0..v.len() {
            let max = match j {
                0 => v[j] + arr[j],
                j if j == v.len() - 1 => v[j] + arr[j-1],
                _ => (v[j] + arr[j-1]).max(v[j] + arr[j])
            };
            tmp.push(max);
        }
        arr = tmp;
    }

    writeln!(output, "{}", arr.iter().max().unwrap())?;
    Ok(())
}