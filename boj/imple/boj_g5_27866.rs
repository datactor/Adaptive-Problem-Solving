use std::{
        io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
        let mut input = String::new();
        let mut output = BufWriter::new(io::stdout().lock());
        io::stdin().lock().read_to_string(&mut input)?;

        let mut iter = input.split_ascii_whitespace();

        let buf = iter.next().unwrap();
        let n = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        if let Some(c) = buf.chars().nth(n) {
                writeln!(output, "{}", c)?;
        }
        Ok(())
}