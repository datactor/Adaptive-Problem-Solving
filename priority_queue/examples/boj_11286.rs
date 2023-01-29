// https://www.acmicpc.net/problem/11286

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::BinaryHeap,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let input = input.split_ascii_whitespace().skip(1);
    let mut hq = BinaryHeap::new();

    for i in input {
        let x = i.parse::<i32>().unwrap();
        match x {
            0 => {
                match hq.is_empty() {
                    true => writeln!(output, "0")?,
                    false => {
                        let mut tmp = BinaryHeap::new();
                        loop {
                            let pop: (i32, i32) = hq.pop().unwrap();

                            tmp.push((pop.1 * -1, pop.0));

                            if hq.peek() == None || pop.0 != hq.peek().unwrap().0 {
                                break
                            };
                        }

                        writeln!(output, "{}", tmp.pop().unwrap().0 * -1)?;

                        for (minus_val, abs) in tmp {
                            hq.push((abs, minus_val * -1));
                        }
                    }
                }
            },
            _ => hq.push((x.abs() * -1, x))
        }
    }

    Ok(())
}