use std::io::{self, prelude::*, BufWriter, StdoutLock};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    // let mut buffer = String::new();
    let mut buffer = String::from("4 2");
    // io::stdin().read_line(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = v.next().unwrap();
    let m = v.next().unwrap();

    let mut arr = Vec::new();

    solve(n, m, &mut arr, &mut output);
}

fn solve(n: usize, m: usize, v: &mut Vec<usize>, output: &mut BufWriter<StdoutLock>) {
    if v.len() == m {
        for i in v {
            write!(output, "{} ", i).unwrap();
        }
        write!(output, "\n").unwrap();
        return
    }

    for i in 1..n+1 {
        if v.contains(&i) {
            continue
        }

        v.push(i);
        solve(n, m, v, output);
        v.pop();
    }
}