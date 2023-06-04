use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    io::stdin().read_line(&mut buffer).unwrap();
    let mut sangun = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    buffer.clear();

    io::stdin().read_line(&mut buffer).unwrap();
    let m = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    io::stdin().read_line(&mut buffer).unwrap();
    let mut cards = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();
    buffer.clear();
    sangun.sort();
    cards.sort_by_key(|&(_, a)| a);

    let mut result = vec![0; m];
    let mut tmp = 0;
    for i in 0..m {
        let mut out = 0;
        while tmp < n {
            if cards[i].1 < sangun[tmp] {
                out = 0;
                break;
            } else if cards[i].1 == sangun[tmp] {
                out = 1;
                tmp += 1;
                break;
            }
            tmp += 1
        }
        result[cards[i].0] = out;
    }

    for i in result {
        write!(output, "{} ", i).unwrap();
    }
}
