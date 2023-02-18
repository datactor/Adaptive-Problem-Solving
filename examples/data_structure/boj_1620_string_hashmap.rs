use std::io::{self, prelude::*, BufWriter};
use std::collections::HashMap;

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace();
    let n = first_line.next().unwrap().parse::<usize>().unwrap();
    let m = first_line.next().unwrap().parse::<usize>().unwrap();

    let mut ordered = Vec::with_capacity(n);
    let mut hashmap = HashMap::with_capacity(n);

    // O(n)
    for (idx, pokemon) in lines.by_ref().take(n).enumerate() {
        ordered.insert(idx, pokemon);
        hashmap.insert(pokemon, idx+1);
    }

    // hashmap get은, vec[idx]처럼 key값을 인덱스처럼 변환하여 빠른접근 가능. O(1)인 상수 시간복잡도
    for _ in 0..m {
        let a = lines.next().unwrap();
        if let Ok(idx) = a.parse::<usize>() {
            writeln!(output, "{}", ordered[idx-1]).unwrap();
        } else {
            writeln!(output, "{}", hashmap[a]).unwrap()
        };
        // match a.parse::<usize>() {
        //     Ok(idx) => {
        //         writeln!(output, "{}", ordered[idx-1]).unwrap();
        //     },
        //     Err(E) => {
        //         writeln!(output, "{}", hashmap.get(a).unwrap()).unwrap();
        //     },
        // };
    }
}