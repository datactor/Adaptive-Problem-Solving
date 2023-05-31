use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    collections::HashSet,
};

macro_rules! read_nums {
    ($reader:expr, $type:ty) => {
        {
            $reader
                .next()
                .expect("Failed to get next line")
                .expect("Failed to read line")
                .split_ascii_whitespace()
                .map(|s| s.parse::<$type>().expect("Failed to parse"))
                .collect::<Vec<$type>>()
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf_to_lines = BufReader::new(io::stdin().lock()).lines();
    let mut write_buf = BufWriter::new(io::stdout().lock());

    let vec = read_nums!(read_buf_to_lines, usize);
    let (n, d, k, c) = (vec[0], vec[1], vec[2], vec[3]);
    let mut seq = (0..n)
        .map(|_| read_buf_to_lines
            .next()
            .expect("Failed to get next line")
            .expect("Failed to read line")
            .trim()
            .parse::<i16>()
            .expect("Failed to parse"))
        .collect::<Vec<i16>>();
    let p = (0..k).map(|s| seq[s]).collect::<Vec<i16>>();
    seq.extend(p);

    // let mut max = 0;
    // let (mut s, mut e) = (0, k);
    //
    // while e < seq.len() {
    //     if max == k+1 { break };
    //     let hash = seq[s..e].iter().cloned().collect::<HashSet<usize>>();
    //     let mut kinds = hash.len();
    //     if !hash.contains(&c) {
    //         kinds += 1
    //     }
    //     max = std::cmp::max(max, kinds);
    //     s += 1;
    //     e += 1;
    // }
    // writeln!(write_buf, "{}", max)?;
    //
    // Ok(())

    let mut cnt = vec![0; d+1];
    let mut kinds = 0;
    for &sushi in &seq[0..k] {
        if cnt[sushi as usize] == 0 {
            kinds += 1;
        }
        cnt[sushi as usize] += 1;
    }

    let mut max = kinds;
    for i in 0..n {
        cnt[seq[i] as usize] -= 1;
        if cnt[seq[i] as usize] == 0 {
            kinds -= 1;
        }
        if cnt[seq[i+k] as usize] == 0 {
            kinds += 1;
        }

        cnt[seq[i+k] as usize] += 1;

        let total = if cnt[c] == 0 {
            kinds + 1
        } else {
            kinds
        };

        max = std::cmp::max(max, total);
    }
    writeln!(write_buf, "{}", max)?;

    Ok(())
}