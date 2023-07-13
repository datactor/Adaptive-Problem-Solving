use std::{
    io::{self, BufRead, BufReader, Write, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(io::stdin().lock());
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();
    let mut total_credits = 0.0;
    let mut total_score = 0.0;
    for _ in 0..20 {
        buf_to_string.clear();
        buf_reader.read_line(&mut buf_to_string)?;
        let mut iter = buf_to_string.split_ascii_whitespace();
        iter.next();
        let credits = iter.next().unwrap().parse::<f64>()?;
        total_credits += credits;
        let score = match iter.next().unwrap().as_bytes() {
            b"A+" => 4.5 * credits,
            b"A0" => 4.0 * credits,
            b"B+" => 3.5 * credits,
            b"B0" => 3.0 * credits,
            b"C+" => 2.5 * credits,
            b"C0" => 2.0 * credits,
            b"D+" => 1.5 * credits,
            b"D0" => credits,
            b"F" => 0.0,
            _ => {
                total_credits -= credits;
                0.0
            },
        };
        total_score += score;
    }
    write!(buf_writer, "{:.4}", total_score / total_credits)?;
    Ok(())
}