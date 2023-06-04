// https://www.acmicpc.net/problem/14583

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input)?;

    let v: Vec<f32> = input.split_ascii_whitespace().map(|s| s.parse::<f32>().unwrap()).collect();
    let (w, h) = (v[0], v[1]);
    let d = (w.powf(2.0) + h.powf(2.0)).sqrt();

    let x = (h.powf(2.0) - (d-w).powf(2.0))/(2.0 * h);

    let nw = (w.powf(2.0) + x.powf(2.0)).sqrt() / 2.0;
    let nh = (w * (h - x))/(w.powf(2.0) + x.powf(2.0)).sqrt();

    writeln!(output, "{:.2} {:.2}", nw, nh)?;

    Ok(())
}