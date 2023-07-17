use std::{
    error::Error,
    io::{self, BufRead, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    let n = buffer.trim().parse::<usize>()?;
    for i in 1..=n {
        for _ in 0..(n - i) {
            write!(buf_writer, " ")?;
        }

        for _ in 0..(2 * i - 1) {
            write!(buf_writer, "*")?;
        }

        writeln!(buf_writer)?;
    }

    for i in 1..n {
        for _ in 0..i {
            write!(buf_writer, " ")?;
        }

        for _ in 0..(2 * (n - i) - 1) {
            write!(buf_writer, "*")?;
        }
        writeln!(buf_writer)?;
    }

    Ok(())
}
