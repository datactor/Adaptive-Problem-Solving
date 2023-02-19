use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let mut two = 0;
    let mut five = 0;

    for mut i in (1..n + 1).rev() {
        while true {
            if i % 2 == 0 {
                i /= 2;
                two += 1;
            }
            if i % 5 == 0 {
                i /= 5;
                five += 1;
            }
            if i % 2 != 0 && i % 5 != 0 {
                break;
            }
        }
    }
    writeln!(output, "{}", two.min(five)).unwrap();
}
