use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let k = buffer.trim();
        if let Some(first) = k.chars().next() {
            if let Some(last) = k.chars().last() {
                println!("{}{}", first, last);
            }
        }
    }
}