use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    let mut v = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();

    let n = v.next().unwrap();
    let k = v.next().unwrap();

    // // dp로 풀기
    // let mut arr = vec![vec![0; n+1]; n+1];
    // (arr[1][1], arr[1][0]) = (1, 1);
    //
    // for i in 2..n+1 {
    //     for j in 0..k+1 {
    //         if j == 0 || i == j {
    //             arr[i][j] = 1;
    //         } else {
    //             arr[i][j] = arr[i-1][j-1] % 10007 + arr[i-1][j] % 10007
    //         }
    //     }
    // }
    //
    // writeln!(output, "{}", arr[n][k] % 10007).unwrap();

    // for문으로 풀기
    let mut numer = 1;
    for i in k + 1..=n {
        numer *= i;
        numer %= 10007;
    }
    for i in 2..=n - k {
        let mut denom = 10008;
        while denom % i != 0 {
            denom += 10007;
        }
        denom /= i;
        numer *= denom;
        numer %= 10007;
    }
    writeln!(output, "{}", numer).unwrap();
}
