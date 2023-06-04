use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut iter = input.split_ascii_whitespace();
    let mut next = || iter.next().unwrap().parse::<usize>().unwrap();

    let (n, m) = (next(), next());
    let mut basket = vec![0; n+1];
    for _ in 0..m {
        let (i, j) = (next(), next());
        if basket[i] == 0 {
            basket[i] = i
        }
        if basket[j] == 0 {
            basket[j] = j
        }
        let (a, b) = (basket[i], basket[j]);
        basket[i] = b;
        basket[j] = a;
        // std::mem::swap(&mut basket[i], &mut basket[j]);
    }

    for i in 1..=n {
        write!(output, "{} ", if basket[i] != 0 {
            basket[i]
        } else {
            i
        }
        )?;
    }

    Ok(())
}