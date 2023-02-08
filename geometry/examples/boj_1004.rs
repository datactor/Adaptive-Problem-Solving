use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for _i in 0..n {
        let v = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

        let (x1, y1, x2, y2) = (v[0], v[1], v[2], v[3]);
        let star_len = lines.next().unwrap().parse::<usize>().unwrap();

        let mut sum = 0;
        for _j in 0..star_len {
            let mut star_line = lines.next().unwrap().split_ascii_whitespace().map(
                |s| s.parse::<i32>()).flatten();
            let x = star_line.next().unwrap();
            let y = star_line.next().unwrap();
            let r = star_line.next().unwrap();

            if (x - x1).pow(2) + (y - y1).pow(2) < r.pow(2) &&
                (x - x2).pow(2) + (y - y2).pow(2) < r.pow(2) {
                continue
            } else if (x - x1).pow(2) + (y - y1).pow(2) < r.pow(2) {
                sum += 1;
            } else if (x - x2).pow(2) + (y - y2).pow(2) < r.pow(2) {
                sum += 1;
            }
        }
        writeln!(output, "{}", sum).unwrap();
    }
}