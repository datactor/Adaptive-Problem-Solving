use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input)?;

    let v: Vec<_> = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    let a = (v[0], v[1]);
    let b = (v[2], v[3]);
    let c = (v[4], v[5]);
    let d = (v[6], v[7]);

    let abc = ccw(a, b, c);
    let abd = ccw(a, b, d);
    let cda = ccw(c, d, a);
    let cdb = ccw(c, d, b);

    writeln!(output, "{}", if abc * abd < 0 && cda * cdb < 0 {
        1
    } else {
        0
    })?;

    Ok(())
}

fn ccw(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> i32 {
    let x = (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1);
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}