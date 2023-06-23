// https://www.acmicpc.net/problem/11000

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    error::Error,
    collections::{HashSet, BinaryHeap},
};

macro_rules! read {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.split_ascii_whitespace()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(io::stdin().lock());
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    let n = read!(buf_reader, buf_to_string).next().expect("No input").parse::<usize>().expect("Failed to parse to usize");

    // // O(n) 에 성공할 줄 알았지만 실패
    // let mut hash = HashSet::with_capacity(n);
    // for _ in 0..n {
    //     let mut iter = read!(buf_reader, buf_to_string);
    //     let (s, t) = (iter.next().unwrap().parse::<usize>().unwrap(), iter.next().unwrap().parse::<usize>().unwrap());
    //     hash.insert(t);
    //     hash.remove(&s);
    // }
    //
    // write!(buf_writer, "{}", hash.len())?;

    let mut table = BinaryHeap::with_capacity(n);
    for _ in 0..n {
        let mut iter = read!(buf_reader, buf_to_string);
        let (s, t) = (iter.next().unwrap().parse::<i32>().unwrap(), iter.next().unwrap().parse::<i32>().unwrap());
        table.push((-s, -t))
    }

    let mut hq = BinaryHeap::new();
    hq.push(table.pop().unwrap().1);

    while let Some((s, t)) = table.pop() {
        if let Some(prev_t) = hq.pop() {
            if s > prev_t {
                hq.push(prev_t)
            }
        }
        hq.push(t);
    }

    write!(buf_writer, "{}", hq.len())?;

    Ok(())
}