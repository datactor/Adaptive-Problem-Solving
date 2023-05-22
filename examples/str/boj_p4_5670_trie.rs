// https://www.acmicpc.net/problem/5670

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    collections::HashMap,
};

macro_rules! read_to_num {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let value = $input.trim().parse::<$type>().ok();
            value
        }
    };
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

    fn insert(&mut self, s: Vec<u8>) {
        let mut current = self;
        for b in s {
            current = current.children.entry(b).or_insert(Trie::new());
        }
        current.end = true;
    }

    fn find(&self, s: &[u8], k: usize, root: bool) -> usize {
        let mut k = k;
        if s.is_empty() {
            return k;
        }
        let (b, rest) = s.split_at(1);
        let b = b.iter().next().unwrap();
        if root {
            k = self.children[b].find(rest, k, false);
        } else if self.children.len() == 1 && !self.end {
            k = self.children[b].find(rest, k, false);
        } else {
            k = self.children[b].find(rest, k + 1, false);
        }
        k
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    while let Some(n) = read_to_num!(reader, buffer, usize) {
        let mut root = Trie::new();
        let mut ans = 0;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            buffer.clear();
            reader.read_line(&mut buffer)?;
            let s = buffer.trim().as_bytes();
            v.push(s.to_vec());
            root.insert(s.to_vec());
        }

        for s in &v {
            ans += root.find(s, 1, true);
        }
        write!(writer, "{:.2}\n", ans as f64 / v.len() as f64)?;
    }
    Ok(())
}