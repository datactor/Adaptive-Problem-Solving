// https://www.acmicpc.net/problem/9935
// O(n)

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let (s, bomb) = input.trim().split_once('\n').unwrap();
    let bomb = bomb.as_bytes();

    let mut stack = Vec::new();

    for byte in s.bytes() {
        stack.push(byte);
        if stack.len() >= bomb.len() {
            let last = stack.get((stack.len() - bomb.len())..);
            if last == Some(bomb) {
                stack.truncate(stack.len() - bomb.len());
            }
        }
    }

    writeln!(output, "{}", match stack.is_empty() {
        true => "FRULA".to_string(),
        _ => String::from_utf8(stack).unwrap()
    })?;

    Ok(())
}