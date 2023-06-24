// https://www.acmicpc.net/problem/1789

use std::{
    io::{self, Write},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let s = buffer.trim().parse::<f64>()?;
    write!(io::stdout(), "{:.0}", ((2.0 * s + 0.25).sqrt() - 0.5) - 0.499999999)?;

    Ok(())
}