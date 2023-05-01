// https://www.acmicpc.net/problem/9015

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

        let mut vec = Vec::new();
        let mut hashset = HashSet::new();
        for _ in 0..n {
            input.clear();
            reader.read_line(&mut input)?;

            let v: Vec<i32> = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            vec.push((v[0], v[1]));
            hashset.insert((v[0], v[1]));
        }

        let mut ans = 0;

        for i in 0..n-1 {
            for j in i+1..n {
                let a = vec[i].0;
                let b = vec[i].1;
                let c = vec[j].0;
                let d = vec[j].1;
                let dx = c - a;
                let dy = d - b;

                let point1 = (c - dy, d + dx);
                let point2 = (a - dy, b + dx);

                if hashset.get(&point1) != None && hashset.get(&point2) != None {
                    let area = (c - a).pow(2) + (d - b).pow(2);
                    ans = ans.max(area);
                    hashset.remove(&point1);
                    hashset.remove(&point2);
                }
            }
        }
        writeln!(writer, "{}", ans)?;
    }

    Ok(())
}