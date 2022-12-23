use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();

    let mut arr = [[[1; 21]; 21]; 21];

    for i in 1..21 {
        for j in 1..21 {
            for k in 1..21 {
                if i < j && j < k {
                    arr[i][j][k] = arr[i][j][k - 1] + arr[i][j - 1][k - 1] - arr[i][j - 1][k];
                } else {
                    arr[i][j][k] = arr[i-1][j][k] + arr[i-1][j-1][k] + arr[i-1][j][k-1] - arr[i-1][j-1][k-1]
                }
            }
        }
    }
    while true {
        let mut v = lines.next().take().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<i32>()).flatten();
        let (a, b, c) = (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());

        if a == -1 && b == -1 && c == -1 {
            break
        }

        if a <= 0 || b <= 0 || c <= 0 {
            writeln!(output, "w({}, {}, {}) = {}", a, b, c, 1).unwrap();
        } else if a > 20 || b > 20 || c > 20 {
            writeln!(output, "w({}, {}, {}) = {}", a, b, c, arr[20][20][20]).unwrap();
        } else {
            writeln!(output, "w({}, {}, {}) = {}", a, b, c, arr[a as usize][b as usize][c as usize]).unwrap();
        }
    }
}