use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for i in 0..n {
        let mut v = lines.next().unwrap().split_ascii_whitespace().map(
            |s| s.parse::<i32>()).flatten();
        let (x1, y1, r1) =
            (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());
        let (x2, y2, r2) =
            (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());

        let dst = (((x1-x2).pow(2) + (y1-y2).pow(2)) as f32).powf(0.5);
        if dst == 0.0 && r1 == r2 {
            writeln!(output, "-1").unwrap();
        } else if dst == (r1 + r2) as f32 || (r1 - r2).abs() as f32 == dst {
            writeln!(output, "1").unwrap();
        } else if dst < (r1 + r2) as f32 && dst > (r1 - r2).abs() as f32 {
            writeln!(output, "2").unwrap();
        } else {
            writeln!(output, "0").unwrap();
        }
    }
}