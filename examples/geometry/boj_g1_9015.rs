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

        for i in 0..n-3 {
            for j in i+1..n-2 {
                let dist_sq = (vec[j].0 - vec[i].0).pow(2) + (vec[j].1 - vec[i].1).pow(2);
                let x = dist_sq * 2;

                let slope = (vec[j].1 - vec[i].1) / (vec[j].0 - vec[i].0);
                let reciprocal = - 1 as f32 / slope as f32;

                let a = vec[i] as f32 - reciprocal * vec[i] as f32;
                let b = vec[j] as f32 - reciprocal * vec[j] as f32;




            }
        }
    }

    Ok(())
}