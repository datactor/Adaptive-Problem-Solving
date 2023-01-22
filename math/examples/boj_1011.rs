// https://www.acmicpc.net/problem/1011
use std::io::*;

fn main() -> Result<()> {
    let mut output=BufWriter::new(stdout().lock());
    stdin().lock().lines().skip(1)
        .for_each(|s| {
            let mut v: Vec<usize> = s.unwrap().split_ascii_whitespace().map(|s|s.parse().unwrap()).collect();
            let mut d = v[1] - v[0];
            let mut p= 2;
            while d > p / 2 {
                d -= p / 2;
                p += 1;
            }
            writeln!(output, "{}", p-1);
        });

    Ok(())
}