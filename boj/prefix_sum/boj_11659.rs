use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let mut a = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();
    let (n, m) = (a.next().unwrap(), a.next().unwrap());

    let mut arr = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // // 시간초과
    // for _ in 0..m {
    //     let mut indexes = lines.next().unwrap().split_ascii_whitespace().map(
    //         |s| s.parse::<usize>()).flatten();
    //     let (il, ir) = (indexes.next().unwrap() - 1, indexes.next().unwrap());
    //     writeln!(output, "{}", arr[il..ir].iter().sum::<usize>()).unwrap();
    // }

    // dp로 풀기
    let mut array = vec![0; n + 1];
    for i in 0..n {
        array[i + 1] = array[i] + arr[i]
    }

    for i in 0..m {
        let mut indexes = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>())
            .flatten();
        let (il, ir) = (indexes.next().unwrap() - 1, indexes.next().unwrap());
        writeln!(output, "{}", array[ir] - array[il]).unwrap();
    }
}
