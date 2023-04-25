// https://www.acmicpc.net/problem/10655
// O(n)
// Manhattan distance

use std::io::{self, prelude::*, BufReader, BufWriter};

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input)?;

    let n = input.trim().parse::<usize>().unwrap();

    let mut pre = (i32::MIN, i32::MIN);
    let mut cur = (i32::MIN, i32::MIN);

    let mut pre_dist = 0;
    let mut dist = 0;

    let mut total = 0;
    let mut min = i32::MAX;

    for i in 0..n {
        input.clear();
        reader.read_line(&mut input)?;
        let mut iter = input.split_ascii_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();

        let pre_check = (pre_dist, dist);

        if pre != (i32::MIN, i32::MIN) {
            pre_dist = (x - pre.0).abs() + (y - pre.1).abs();
        }

        if i != 0 {
            dist = (x - cur.0).abs() + (y - cur.1).abs();
            pre = cur;
        }

        cur = (x, y);
        total += dist;

        if 1 < i && i < n {
            min = min.min(pre_dist - dist - pre_check.1);
        }
    }

    writeln!(writer, "{}", total + min)?;

    Ok(())
}