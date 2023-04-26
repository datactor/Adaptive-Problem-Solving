use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut v = input.split_ascii_whitespace();
    let x = v.next().unwrap().parse::<i32>().unwrap() > 0;
    let y = v.next().unwrap().parse::<i32>().unwrap() > 0;

    writeln!(output, "{}",
             if x {
                 if y {
                     1
                 } else {
                     4
                 }
             } else {
                 if y {
                     2
                 } else {
                     3
                 }
             }
    )?;
    Ok(())
}