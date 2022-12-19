use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = v.next().unwrap();
    let k = v.next().unwrap();

    let mut arr = vec![vec![0; n+1]; n+1];
    (arr[1][1], arr[1][0]) = (1, 1);

    for i in 2..n+1 {
        for j in 0..k+1 {
            if j == 0 || i == j {
                arr[i][j] = 1;
            } else {
                arr[i][j] = arr[i-1][j-1] % 10007 + arr[i-1][j] % 10007
            }
        }
    }

    writeln!(output, "{}", arr[n][k] % 10007).unwrap();
}