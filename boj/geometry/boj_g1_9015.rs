// https://www.acmicpc.net/problem/9015
// O(n.pow(2))

use std::{
    io::{self, prelude::*, BufReader, BufWriter},
    collections::HashSet,
};

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input)?;

    let t = input.trim().parse::<usize>().unwrap();
    input.clear();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input)?;
        let n = input.trim().parse::<usize>().unwrap();

        let mut x_coords = Vec::new();
        let mut y_coords = Vec::new();
        let mut point_set = HashSet::new();
        for _ in 0..n {
            input.clear();
            reader.read_line(&mut input)?;

            let v: Vec<i32> = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            let x = v[0];
            let y = v[1];
            x_coords.push(x);
            y_coords.push(y);
            point_set.insert((x, y));
        }

        let mut ans = 0;

        for i in 0..n-1 {
            for j in i+1..n {
                let a = x_coords[i];
                let b = y_coords[i];
                let c = x_coords[j];
                let d = y_coords[j];
                let dx = c - a;
                let dy = d - b;

                let point1 = (c - dy, d + dx);
                let point2 = (a - dy, b + dx);

                if point_set.contains(&point1) && point_set.contains(&point2) {
                    let area = dx.pow(2) + dy.pow(2);
                    ans = ans.max(area);
                    point_set.remove(&point1);
                    point_set.remove(&point2);
                }
            }
        }
        writeln!(writer, "{}", ans)?;
    }

    Ok(())
}