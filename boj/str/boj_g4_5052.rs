// https://www.acmicpc.net/problem/5052

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    collections::HashMap,
};

fn read_to_num(reader: &mut dyn BufRead, buffer: &mut String) -> io::Result<usize> {
    buffer.clear();
    reader.read_line(buffer)?;

    Ok(buffer.trim().parse::<usize>().expect("Failed to parse"))
}

struct Trie {
    children: HashMap<u8, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            end: false,
        }
    }

    fn insert(&mut self, s: Vec<u8>) -> bool {
        let mut current = self;
        for b in s {
            if current.end {
                // This string is a prefix of some string
                return false;
            }

            current = current.children.entry(b).or_insert(Trie::new());
        }

        if !current.children.is_empty() {
            // There exists a string of which this string is a prefix
            return false;
        }

        current.end = true;
        true
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    let t = read_to_num(&mut read_buf, &mut buf_to_string)?;
    for _ in 0..t {
        let n = read_to_num(&mut read_buf, &mut buf_to_string)?;
        let mut root = Trie::new();
        let mut vec = Vec::with_capacity(n);
        let mut is_consistent = true;
        for _ in 0..n {
            buf_to_string.clear();
            read_buf.read_line(&mut buf_to_string)?;

            let address = buf_to_string.trim().as_bytes().to_vec();
            // if !root.insert(address) {
            //     is_consistent = false;
            // }
            vec.push(address);
        }
        vec.sort();

        for i in 1..n {
            let len = vec[i-1].len();
            if len <= vec[i].len() && vec[i-1][0..len] == vec[i][0..len] {
                is_consistent = false;
                break
            }
        }

        writeln!(write_buf, "{}", if is_consistent {
            "YES"
        } else {
            "NO"
        })?;
    }

    Ok(())
}