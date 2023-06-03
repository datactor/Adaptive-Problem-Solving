// https://www.acmicpc.net/problem/2096

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    cmp,
};

macro_rules! get_min_max_points {
    ($reader:expr, $writer:expr, $type:ty) => {
        {
            let n = $reader.next().expect("Failed to get next line").expect("Failed to read line").parse::<usize>().expect("Failed to parse");
            let mut min_table = [0; 3];
            let mut max_table = [0; 3];

            let mut xyz = [0; 3];

            for i in 0..n {
                let line = $reader.next().expect("Failed to get next line").expect("Failed to read");
                let mut iter = line.split_ascii_whitespace();
                xyz[0] = iter.next().expect("Failed to get next iter").parse::<$type>().expect("Failed to parse");
                xyz[1] = iter.next().expect("Failed to get next iter").parse::<$type>().expect("Failed to parse");
                xyz[2] = iter.next().expect("Failed to get next iter").parse::<$type>().expect("Failed to parse");

                if i == 0 {
                    min_table = [xyz[0], xyz[1], xyz[2]];
                    max_table = [xyz[0], xyz[1], xyz[2]];
                    continue
                }

                let mut mn = [0, 0, 0];
                let mut mx = [0, 0, 0];
                mn[0] = cmp::min(min_table[0], min_table[1]);
                mn[2] = cmp::min(min_table[1], min_table[2]);
                mn[1] = cmp::min(mn[0], mn[2]);
                mx[0] = cmp::max(max_table[0], max_table[1]);
                mx[2] = cmp::max(max_table[1], max_table[2]);
                mx[1] = cmp::max(mx[0], mx[2]);
                for i in 0..3 {
                    min_table[i] = mn[i] + xyz[i];
                    max_table[i] = mx[i] + xyz[i];
                }
            }
            writeln!($writer, "{} {}", max_table.iter().max().unwrap(), min_table.iter().min().unwrap())?;
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());
    get_min_max_points!(read_buf, write_buf, usize);
    Ok(())
}