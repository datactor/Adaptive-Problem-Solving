use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let mut arr = [[0; 31]; 31];
    for i in 1..31 {
        for j in i..31 {
            if i == 1 {
                arr[i][j] = j;
            } else if i == j {
                arr[i][j] = 1;
            } else {
                arr[i][j] = arr[i-1][j-1] + arr[i][j-1]
            }
        }
    }

    let t = v.next().unwrap();
    for _ in 0..t {
        let n = v.next().unwrap();
        let m = v.next().unwrap();
        writeln!(output, "{}", arr[n][m]).unwrap();
    }
}