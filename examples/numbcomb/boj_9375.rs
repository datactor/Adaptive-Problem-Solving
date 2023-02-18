use std::io::{self, prelude::*, BufWriter};
use std::collections::HashMap;

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<String>()).flatten();

    let t = v.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut hash = HashMap::new();
        let n = v.next().unwrap().parse::<usize>().unwrap();

        for i in 0..n {
            v.next().unwrap();
            let word = v.next().unwrap();
            let count = hash.entry(word).or_insert(0);
            *count += 1;
        }
        if hash.len() == 1 {
            // writeln!(output, "{}", hash.get(hash.keys().next().unwrap()).unwrap()).unwrap();
            writeln!(output, "{}", hash.values().next().unwrap()).unwrap();
        } else {
            let mut ans = 1;
            for (k, num) in hash {
                ans *= num + 1;
            }
            writeln!(output, "{}", ans - 1).unwrap();
        }
    }
}