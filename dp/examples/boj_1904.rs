use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    // dp로 풀기
    let mut arr = [0; 1000001];
    (arr[1], arr[2]) = (1, 2);

    for i in 3..n+1 {
        arr[i] = (arr[i-1] + arr[i-2]) % 15746
    }
    writeln!(output, "{}", arr[n]).unwrap();
}